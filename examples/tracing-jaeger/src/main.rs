use opentelemetry::trace::SpanContext;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::Context;
use opentelemetry_aws::trace::xray_propagator::span_context_from_str;
use std::error::Error;

use opentelemetry_sdk::trace::TracerProvider;

use tracing::{info, info_span};
// For using subscriber.with()
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, fmt::format, EnvFilter};
// For using span.context()
use tracing_opentelemetry::OpenTelemetrySpanExt;
// For using Context.with_remote_span_context()
use opentelemetry::trace::TraceContextExt;
use tracing_subscriber::fmt::writer::MakeWriterExt;

fn init_observability() {
    let exporter = opentelemetry_stdout::SpanExporter::default();
    // Create a new OpenTelemetry trace pipeline that prints to stdout
    let provider = TracerProvider::builder()
        .with_simple_exporter(exporter)
        .build();
    let tracer = provider.tracer("lambda");
    // let fmt_layer = fmt::layer().event_format(format().json());
    // let filter_layer = EnvFilter::from_default_env();
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        // .with(fmt_layer)
        .with(telemetry_layer)
        .init();
}

fn retrieve_span_context_from_xray_traceid(xray_trace_id: &str) -> SpanContext {
    span_context_from_str(xray_trace_id).expect("cannot extract")
}

#[tokio::main]
async fn main() {
    let provider = init_observability();

    let xray_trace_id = "Root=1-65dc5008-1561ed7046ffcbcb114af027;Parent=b510129166d5a083;Sampled=1;Lineage=f98dd9ff:0";
    let parent_spancontext = retrieve_span_context_from_xray_traceid(&xray_trace_id);

    let ctx = Context::current().with_remote_span_context(parent_spancontext);
    let span = tracing::span!(tracing::Level::ERROR, "app_start");
    span.set_parent(ctx);
    let _enter = span.enter();


    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}

