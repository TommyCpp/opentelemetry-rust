use opentelemetry::{
    trace::{Span, Tracer, TracerProvider as _},
    KeyValue,
};

use opentelemetry_sdk::{
    resource,
    trace::{config, Config, TracerProvider},
    Resource,
};
use opentelemetry_stdout::SpanExporterBuilder;
use tracing::span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{layer::SubscriberExt, Registry};

fn main() {
    println!("Hello, world!");
    let stdout_exporter = SpanExporterBuilder::default()
        .with_encoder(|writer, data| {
            serde_json::to_writer_pretty(writer, &data).unwrap();
            Ok(())
        })
        .build();
    let provider = TracerProvider::builder()
        .with_config(Config::default().with_resource(Resource::new(vec![])))
        .with_simple_exporter(stdout_exporter)
        .build();
    let tracer = provider.tracer("tracing-opentelemetry");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);

    tracing::subscriber::with_default(subscriber, || {
        let tracer = provider.tracer("otel-tracing");

        // this is the root span created using tokio tracing api
        let root = span!(tracing::Level::TRACE, "tokio-tracing-span-parent");
        let _enter = root.enter();

        // this is a child span created using tokio tracing api
        // this correctly parented to the root span
        let child = span!(tracing::Level::TRACE, "tokio-tracing-span-child");
        let _enter_child = child.enter();

        // this is another child, created using otel tracing api
        // but this is *NOT* parented to the above spans
        // demonstrating broken traces when mixing and matching tracing and opentelemetry apis
        // let mut span = tracer
        //     .start_with_context("otel-tracing-span", &child.context());
        let mut span = tracer
            .start("otel-tracing-span");
        span.end();
    });
}
