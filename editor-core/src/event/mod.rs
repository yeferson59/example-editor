//! Event system module
//!
//! Provides event handling and dispatching mechanisms for editor state changes

use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};

/// Represents different types of editor events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /// Document-related events
    Document(DocumentEvent),
    /// Buffer-related events
    Buffer(BufferEvent),
    /// Editor state events
    Editor(EditorEvent),
}

/// Document-specific events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentEvent {
    /// Document was opened
    Opened {
        path: Option<std::path::PathBuf>,
        name: String,
    },
    /// Document was saved
    Saved {
        path: std::path::PathBuf,
    },
    /// Document was closed
    Closed {
        name: String,
    },
    /// Document language changed
    LanguageChanged {
        name: String,
        language: Option<String>,
    },
}

/// Buffer-specific events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BufferEvent {
    /// Text was inserted
    Inserted {
        position: usize,
        text: String,
    },
    /// Text was deleted
    Deleted {
        start: usize,
        end: usize,
        text: String,
    },
    /// Buffer was modified
    Modified {
        dirty: bool,
    },
}

/// Editor state events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditorEvent {
    /// Editor mode changed
    ModeChanged {
        mode: String,
    },
    /// Editor theme changed
    ThemeChanged {
        theme: String,
    },
    /// Editor configuration changed
    ConfigChanged {
        key: String,
        value: String,
    },
}

/// Trait for handling editor events
///
/// # Examples
///
/// ```
/// use editor_core::event::{EventDispatcher, Event, EventHandler};
/// use std::sync::Arc;
///
/// struct MyHandler;
///
/// #[async_trait::async_trait]
/// impl EventHandler for MyHandler {
///     async fn handle(&self, event: Event) {
///         println!("Received event: {:?}", event);
///     }
/// }
///
/// let dispatcher = EventDispatcher::new();
/// let handler = Arc::new(MyHandler);
///
/// let subscription = dispatcher.subscribe().with_handler(handler);
///
/// // Dispatch an event
/// dispatcher.dispatch(Event::Document(DocumentEvent::Opened {
///     path: Some("example.txt".into()),
///     name: "example.txt".to_string(),
/// }));
/// ```
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle an editor event
    async fn handle(&self, event: Event);
}

/// Event dispatcher for broadcasting events to registered handlers
///
/// # Examples
///
/// ```
/// use editor_core::event::{EventDispatcher, Event, DocumentEvent, EventHandler};
/// use std::sync::Arc;
/// use async_trait::async_trait;
///
/// struct MyHandler;
///
/// #[async_trait]
/// impl EventHandler for MyHandler {
///     async fn handle(&self, event: Event) {
///         println!("Received event: {:?}", event);
///     }
/// }
///
/// let dispatcher = EventDispatcher::new();
/// let handler = Arc::new(MyHandler);
///
/// let subscription = dispatcher.subscribe()
///     .with_handler(handler);
///
/// // In a real application, you would spawn this to run in the background
/// // tokio::spawn(subscription.listen());
///
/// // Dispatch an event
/// dispatcher.dispatch(Event::Document(DocumentEvent::Opened {
///     path: Some("example.txt".into()),
///     name: "example.txt".to_string(),
/// }));
/// ```
#[allow(dead_code)]
pub struct EventDispatcher {
    /// Channel for broadcasting events
    sender: broadcast::Sender<Event>,
}

#[allow(dead_code)]
impl EventDispatcher {
    /// Creates a new event dispatcher
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    /// Dispatches an event to all registered handlers
    pub fn dispatch(&self, event: Event) {
        let _ = self.sender.send(event);
    }

    /// Subscribes to events
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper struct for managing event subscriptions
///
/// This struct provides a convenient way to manage event subscriptions
/// and their associated handlers.
///
/// # Examples
///
/// ```
/// use editor_core::event::{EventDispatcher, EventSubscription, EventHandler, Event};
/// use std::sync::Arc;
/// use async_trait::async_trait;
///
/// struct MyHandler;
///
/// #[async_trait]
/// impl EventHandler for MyHandler {
///     async fn handle(&self, event: Event) {
///         println!("Received event: {:?}", event);
///     }
/// }
///
/// let dispatcher = EventDispatcher::new();
/// let subscription = EventSubscription::new(dispatcher.subscribe())
///     .with_handler(Arc::new(MyHandler));
///
/// // Start listening for events (this would typically be spawned as a task)
/// // tokio::spawn(subscription.listen());
/// ```
#[allow(dead_code)]
pub struct EventSubscription {
    /// The event receiver
    receiver: broadcast::Receiver<Event>,
    /// Optional event handler
    handler: Option<Arc<dyn EventHandler>>,
}

#[allow(dead_code)]
impl EventSubscription {
    /// Creates a new event subscription
    pub fn new(receiver: broadcast::Receiver<Event>) -> Self {
        Self {
            receiver,
            handler: None,
        }
    }

    /// Sets an event handler for this subscription
    pub fn with_handler(mut self, handler: Arc<dyn EventHandler>) -> Self {
        self.handler = Some(handler);
        self
    }

    /// Starts listening for events
    pub async fn listen(mut self) {
        while let Ok(event) = self.receiver.recv().await {
            if let Some(handler) = &self.handler {
                handler.handle(event).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct TestHandler {
        received: Arc<parking_lot::RwLock<Vec<Event>>>,
    }

    #[async_trait]
    impl EventHandler for TestHandler {
        async fn handle(&self, event: Event) {
            self.received.write().push(event);
        }
    }

    #[tokio::test]
    async fn test_event_system() {
        let dispatcher = EventDispatcher::new();
        let received = Arc::new(parking_lot::RwLock::new(Vec::new()));
        
        let handler = Arc::new(TestHandler {
            received: received.clone(),
        });

        let subscription = EventSubscription::new(dispatcher.subscribe())
            .with_handler(handler);

        // Spawn the listener
        let _handle = tokio::spawn(subscription.listen());

        // Dispatch some events
        dispatcher.dispatch(Event::Document(DocumentEvent::Opened {
            path: None,
            name: "test.txt".to_string(),
        }));

        // Allow some time for event processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Verify events were received
        let events = received.read();
        assert_eq!(events.len(), 1);
        match &events[0] {
            Event::Document(DocumentEvent::Opened { name, .. }) => {
                assert_eq!(name, "test.txt");
            }
            _ => panic!("Unexpected event type"),
        }
    }

    #[tokio::test]
    async fn test_event_dispatch() {
        let dispatcher = EventDispatcher::new();
        let received = Arc::new(parking_lot::RwLock::new(Vec::new()));
        
        let handler = Arc::new(TestHandler {
            received: Arc::clone(&received),
        });

        let subscription = EventSubscription::new(dispatcher.subscribe())
            .with_handler(handler);

        // Spawn the event listener
        let listen_handle = tokio::spawn(async move {
            subscription.listen().await;
        });

        // Dispatch some events
        dispatcher.dispatch(Event::Document(DocumentEvent::Opened {
            path: Some("test.txt".into()),
            name: "test.txt".to_string(),
        }));

        dispatcher.dispatch(Event::Buffer(BufferEvent::Modified {
            dirty: true,
        }));

        // Allow events to be processed
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Check received events
        let events = received.read();
        assert_eq!(events.len(), 2);

        // Ensure events were received in the right order
        match &events[0] {
            Event::Document(DocumentEvent::Opened { name, .. }) => {
                assert_eq!(name, "test.txt");
            }
            _ => panic!("Unexpected first event type"),
        }

        match &events[1] {
            Event::Buffer(BufferEvent::Modified { dirty }) => {
                assert_eq!(*dirty, true);
            }
            _ => panic!("Unexpected second event type"),
        }

        // Cleanup
        listen_handle.abort();
    }
}
