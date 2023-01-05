use tokio::sync::{
    mpsc::{channel, error::SendError, Receiver, Sender},
    Mutex,
};

pub(super) struct BoundedQueue<T> {
    tx: Sender<T>,
    rx: Mutex<Receiver<T>>,
}

impl<T> BoundedQueue<T> {
    pub(super) fn new(buffer: usize) -> Self {
        let (tx, rx) = channel(buffer);

        Self {
            tx,
            rx: Mutex::new(rx),
        }
    }

    pub(super) async fn enqueue(&self, value: T) -> Result<(), SendError<T>> {
        self.tx.send(value).await
    }

    pub(super) async fn dequeue(&self) -> Option<T> {
        self.rx.lock().await.recv().await
    }
}
