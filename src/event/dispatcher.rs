//! Event dispatcher implementation

use super::{EditorEvent, EventHandler, HandlerId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use parking_lot::Mutex;

/// Dispatches events to registered handlers
pub struct EventDispatcher {
    /// Registered event handlers
    handlers: Arc<RwLock<HashMap<HandlerId, Box<dyn EventHandler>>>>,
    /// Next handler ID
    next_id: Arc<Mutex<HandlerId>>,
}

impl EventDispatcher {
    /// Creates a new event dispatcher
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(0)),
        }
    }

    /// Registers an event handler
    pub async fn register(&self, handler: Box<dyn EventHandler>) -> HandlerId {
        let id = {
            let mut next_id = self.next_id.lock();
            let id = *next_id;
            *next_id += 1;
            id
        };

        self.handlers.write().await.insert(id, handler);
        id
    }

    /// Unregisters an event handler
    pub async fn unregister(&self, id: HandlerId) {
        self.handlers.write().await.remove(&id);
    }

    /// Dispatches an event to all registered handlers
    pub async fn dispatch(&self, event: EditorEvent) {
        let handlers = self.handlers.read().await;
        for handler in handlers.values() {
            handler.handle_event(event.clone()).await;
        }
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{BufferEvent, EditorEvent};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestHandler {
        counter: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl EventHandler for TestHandler {
        async fn handle_event(&self, _event: EditorEvent) {
            self.counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[tokio::test]
    async fn test_event_dispatch() {
        let dispatcher = EventDispatcher::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let handler = TestHandler {
            counter: counter.clone(),
        };

        let id = dispatcher.register(Box::new(handler)).await;

        let event = EditorEvent::Buffer(BufferEvent::Modified { modified: true });
        dispatcher.dispatch(event).await;

        assert_eq!(counter.load(Ordering::SeqCst), 1);

        dispatcher.unregister(id).await;

        let event = EditorEvent::Buffer(BufferEvent::Modified { modified: false });
        dispatcher.dispatch(event).await;

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
