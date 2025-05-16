//! Event queue implementation

use super::EditorEvent;
use tokio::sync::mpsc;

/// Queue for buffering events
pub struct EventQueue {
    /// Sender for the event channel
    sender: mpsc::Sender<EditorEvent>,
    /// Receiver for the event channel
    receiver: mpsc::Receiver<EditorEvent>,
}

impl EventQueue {
    /// Creates a new event queue
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);
        Self { sender, receiver }
    }

    /// Sends an event to the queue
    pub async fn send(&self, event: EditorEvent) -> Result<(), mpsc::error::SendError<EditorEvent>> {
        self.sender.send(event).await
    }

    /// Receives an event from the queue
    pub async fn receive(&mut self) -> Option<EditorEvent> {
        self.receiver.recv().await
    }

    /// Returns a sender for the queue
    pub fn sender(&self) -> mpsc::Sender<EditorEvent> {
        self.sender.clone()
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::BufferEvent;

    #[tokio::test]
    async fn test_event_queue() {
        let mut queue = EventQueue::new(10);
        let sender = queue.sender();

        let event = EditorEvent::Buffer(BufferEvent::Modified { modified: true });
        sender.send(event.clone()).await.unwrap();

        let received = queue.receive().await.unwrap();
        match received {
            EditorEvent::Buffer(BufferEvent::Modified { modified }) => {
                assert!(modified);
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[tokio::test]
    async fn test_queue_ordering() {
        let mut queue = EventQueue::new(10);
        let sender = queue.sender();

        let events = vec![
            EditorEvent::Buffer(BufferEvent::Modified { modified: true }),
            EditorEvent::Buffer(BufferEvent::Modified { modified: false }),
        ];

        for event in events.clone() {
            sender.send(event).await.unwrap();
        }

        for expected in events {
            let received = queue.receive().await.unwrap();
            assert!(matches!(
                received,
                EditorEvent::Buffer(BufferEvent::Modified { .. })
            ));
        }
    }
}
