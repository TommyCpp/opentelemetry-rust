#[cfg(feature = "full")]
#[allow(warnings)]
mod tests {
    use async_trait::async_trait;
    use bytes::Bytes;
    use http::{Request, Response};
    use opentelemetry::runtime;
    use opentelemetry::sdk::trace::{BatchConfig, BatchSpanProcessor};
    use opentelemetry::trace::{SamplingResult, Span, TraceState, Tracer, TracerProvider};
    use opentelemetry_http::{HttpClient, HttpError};
    use std::sync::atomic::AtomicU64;
    use std::sync::Arc;
    use std::time;
    use std::time::Duration;

    // http client that will do nothing but pretend the remote is hanging forever
    #[derive(Debug)]
    struct HangingHttpClient {
        counter: Arc<AtomicU64>,
    }

    impl HangingHttpClient {
        fn new() -> Self {
            Self {
                counter: Arc::new(AtomicU64::new(0)),
            }
        }

        fn request_count(&self) -> u64 {
            self.counter.load(std::sync::atomic::Ordering::SeqCst)
        }
    }

    impl Clone for HangingHttpClient {
        fn clone(&self) -> Self {
            HangingHttpClient {
                counter: self.counter.clone(),
            }
        }
    }
    #[async_trait]
    impl HttpClient for HangingHttpClient {
        async fn send(&self, _: Request<Vec<u8>>) -> Result<Response<Bytes>, HttpError> {
            tokio::time::sleep(Duration::from_secs(60 * 60)).await;
            self.counter
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(Response::new(Bytes::new()))
        }
    }

    #[test]
    fn remote_endpoint_failure() {

    }
}
