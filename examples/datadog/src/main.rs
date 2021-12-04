use std::thread;
use std::time::Duration;

use bytes::Bytes;
use http::{Request, Response};
use opentelemetry::{
    Key,
    trace::{Span, TraceContextExt, Tracer},
};
use opentelemetry::global;
use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::runtime::Tokio;
use opentelemetry::sdk::trace;
use opentelemetry::sdk::trace::Sampler;
use opentelemetry_datadog::{ApiVersion, new_pipeline};
use opentelemetry_http::{HttpClient, HttpError};
use tracing::span;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

// Start a datadog agent using docker
// docker run --rm -d\
//     -v /var/run/docker.sock:/var/run/docker.sock:ro \
//     -v /proc/:/host/proc/:ro \
//     -v /sys/fs/cgroup/:/host/sys/fs/cgroup:ro \
//     -p 127.0.0.1:8126:8126/tcp \
//     -e DD_API_KEY="<API_TOKEN>" \
//     -e DD_APM_ENABLED=true \
//     datadog/agent:latest
//

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let tracer = new_pipeline()
        .with_trace_config(trace::config().with_sampler(Sampler::AlwaysOn))
        .with_service_name("test")
        .with_version(ApiVersion::Version05)
        .with_agent_endpoint("http://localhost:8126")
        .install_batch(Tokio)
        .expect("failed to initialize tracing pipeline");

    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .with_target(false)
        .finish()
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    span!(tracing::Level::INFO, "expensive_step_1");

    shutdown_tracer_provider();

    Ok(())
}
