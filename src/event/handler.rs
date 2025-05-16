//! Event handler trait and implementations

use super::EditorEvent;
use async_trait::async_trait;

/// Handler ID type
pub type HandlerId = u64;

/// Event handler trait
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handles an event
    async fn handle_event(&self, event: EditorEvent);
}

/// Event handler that logs events
pub struct LoggingHandler;

#[async_trait]
impl EventHandler for LoggingHandler {
    async fn handle_event(&self, event: EditorEvent) {
        log::info!("Event: {:?}", event);
    }
}

/// Event handler that filters events
pub struct FilteringHandler<H> {
    /// Inner handler
    inner: H,
    /// Filter function
    filter: Box<dyn Fn(&EditorEvent) -> bool + Send + Sync>,
}

impl<H: EventHandler> FilteringHandler<H> {
    /// Creates a new filtering handler
    pub fn new<F>(inner: H, filter: F) -> Self
    where
        F: Fn(&EditorEvent) -> bool + Send + Sync + 'static,
    {
        Self {
            inner,
            filter: Box::new(filter),
        }
    }
}

#[async_trait]
impl<H: EventHandler> EventHandler for FilteringHandler<H> {
    async fn handle_event(&self, event: EditorEvent) {
        if (self.filter)(&event) {
            self.inner.handle_event(event).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{BufferEvent, EditorEvent};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct CountingHandler {
        counter: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl EventHandler for CountingHandler {
        async fn handle_event(&self, _event: EditorEvent) {
            self.counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[tokio::test]
    async fn test_filtering_handler() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counting_handler = CountingHandler {
            counter: counter.clone(),
        };

        let filtering_handler = FilteringHandler::new(counting_handler, |event| {
            matches!(event, EditorEvent::Buffer(_))
        });

        // Buffer event should be handled
        filtering_handler
            .handle_event(EditorEvent::Buffer(BufferEvent::Modified { modified: true }))
            .await;
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // UI event should be filtered out
        filtering_handler
            .handle_event(EditorEvent::Ui(crate::event::UiEvent::ThemeChange {
                theme: "dark".to_string(),
            }))
            .await;
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
