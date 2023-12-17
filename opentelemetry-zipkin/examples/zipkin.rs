use opentelemetry::{
    global::{self, shutdown_tracer_provider},
    trace::{Span, Tracer},
};
use std::thread;
use std::time::Duration;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use opentelemetry_http::hyper::HyperClient;

fn bar() {
    let tracer = global::tracer("component-bar");
    let mut span = tracer.start("bar");
    thread::sleep(Duration::from_millis(6));
    span.end()
}

fn build_http_client(duration: Duration) -> HyperClient<HttpConnector>{
    let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new())
        .build_http();

    HyperClient::new_with_timeout(client, duration)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {

    let tracer = opentelemetry_zipkin::new_pipeline()
        .with_service_name("trace-demo")
        .with_http_client_timeout(build_http_client)
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    tracer.in_span("foo", |_cx| {
        thread::sleep(Duration::from_millis(6));
        bar();
        thread::sleep(Duration::from_millis(6));
    });

    shutdown_tracer_provider();
    Ok(())
}
