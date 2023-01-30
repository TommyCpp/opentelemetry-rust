/*
Notes:
1. need to run the following example for longer time to observe if there is a memory leak.
2. maybe take a look at https://github.com/tokio-rs/tokio/issues/2637? Seems to be related.
 */


use async_trait::async_trait;
use opentelemetry::sdk::trace::BatchConfig;
use opentelemetry::trace::{SamplingResult, Span, TraceState, Tracer};
use opentelemetry_http::HttpClient;
use std::time::Duration;
use tokio::time::Instant;

#[derive(Debug)]
struct HangingHttpClient;

#[async_trait]
impl HttpClient for HangingHttpClient {
    async fn send(
        &self,
        _: http::Request<Vec<u8>>,
    ) -> Result<http::Response<bytes::Bytes>, opentelemetry_http::HttpError> {
        tokio::time::sleep(Duration::from_secs(60 * 60)).await;
        Ok(http::Response::new(bytes::Bytes::new()))
    }
}

#[allow(warnings)]
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("cannot start tokio runtime")
        .block_on(async move {
            let tracer = opentelemetry_jaeger::new_collector_pipeline()
                .with_endpoint("http://localhost:14268/api/traces")
                .with_service_name("test")
                .with_http_client(HangingHttpClient)
                .with_timeout(Duration::from_secs(5))
                .with_batch_processor_config(
                    BatchConfig::default()
                        .with_max_export_timeout(Duration::from_secs(5))
                        .with_max_queue_size(5),
                )
                .install_batch(opentelemetry::runtime::Tokio)
                .expect("cannot initialize tracer");

            let current = Instant::now();
            loop {
                let mut builder = tracer.span_builder("test");
                builder.sampling_result = Some(SamplingResult {
                    decision: opentelemetry::trace::SamplingDecision::RecordAndSample,
                    attributes: Vec::new(),
                    trace_state: TraceState::default(),
                });
                let mut span = builder.start(&tracer);
                tokio::time::sleep(Duration::from_secs(1)).await;
                span.end();
                if Instant::now() - current > Duration::from_secs(60 * 5) {
                    break;
                }
            }
        });
}
