//! Internal utilities

/// Helper which wraps `tokio::time::interval` and makes it return a stream
#[cfg(any(test, feature = "rt-tokio", feature = "rt-tokio-current-thread"))]
pub fn tokio_interval_stream(
    period: std::time::Duration,
) -> tokio_stream::wrappers::IntervalStream {
    tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(period))
}

pub mod channel {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use futures::{
        channel::mpsc::{UnboundedSender, UnboundedReceiver, unbounded},
        Stream,
    };
    use std::task::{Context, Poll};
    use std::pin::Pin;

    pub fn bounded_channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (unbounded_sender, unbounded_receiver) = unbounded::<T>();
        let count = Arc::new(AtomicUsize::new(0));
        (Sender {
            capacity,
            count: count.clone(),
            unbounded_sender,
        }, Receiver {
            count,
            unbounded_receiver,
        })
    }

    #[derive(Debug)]
    pub struct Sender<T> {
        count: Arc<AtomicUsize>,
        capacity: usize,
        unbounded_sender: UnboundedSender<T>,
    }

    impl<T> Sender<T> {
        pub fn try_send(&self, item: T) -> Result<(), ()> {
            if self.count.load(Ordering::Relaxed) < self.capacity {
                self.count.fetch_add(1, Ordering::Relaxed);
                self.unbounded_sender.unbounded_send(item);
                Ok(())
            } else {
                Err(())
            }
        }
    }

    #[derive(Debug)]
    pub struct Receiver<T> {
        count: Arc<AtomicUsize>,
        unbounded_receiver: UnboundedReceiver<T>,
    }

    impl<T> Stream for Receiver<T> {
        type Item = T;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let mut this = self.as_mut();
            let poll = Pin::new(&mut this.unbounded_receiver).poll_next(cx);
            if let Poll::Ready(Some(item)) = poll {
                self.count.fetch_sub(1, Ordering::Relaxed);
                Poll::Ready(Some(item))
            } else {
                poll
            }
        }
    }


    #[cfg(test)]
    mod tests {
        use crate::util::channel::bounded_channel;
        use std::sync::atomic::Ordering;
        use futures::StreamExt;
        use std::time::Duration;

        #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
        async fn test_send_message() {
            let (sender, mut receiver) = bounded_channel(10);
            let handle = tokio::spawn(async move {
                while let Some(item) = receiver.next().await {
                    println!("{}", item);
                }
            });

            for i in 0..12 {
                sender.try_send(i);
            }

            tokio::time::sleep(Duration::from_secs(5)).await;

            for i in 20..38 {
                sender.try_send(i);
            }

            drop(sender);

            handle.await;
        }
    }
}