// src/main.rs
use opentelemetry::trace::{FutureExt, SpanKind, TraceContextExt, Tracer};
use opentelemetry::{Context, KeyValue};
use opentelemetry_sdk::trace::BatchConfigBuilder;
use opentelemetry_sdk::{
    trace::{RandomIdGenerator, Sampler},
    Resource,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_batch_config(
            BatchConfigBuilder::default()
                // .with_max_concurrent_exports(1)
                // shorten the delay between when spans are flushed, default is 5s,
                .with_scheduled_delay(Duration::from_secs(2))
                .build(),
        )
        .with_trace_config(
            opentelemetry_sdk::trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "FooService",
                )])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .unwrap();

    let tracer_provider = tracer.provider().unwrap();

    // NB: This spawn is load bearing
    let _ = tokio::spawn(async move {
        // Do some work
        let span = tracer
            .span_builder(String::from("Greeter/client"))
            .with_kind(SpanKind::Client)
            .with_attributes(vec![KeyValue::new("component", "grpc")])
            .start(&tracer);
        let cx = Context::current_with_span(span);
        let _ = foo().with_context(cx).await;

        // Attempt to flush traces

        println!("start trace_provider flush");
        for r in tracer_provider.force_flush() {
            if let Err(e) = r {
                println!("unable to fully flush traces: {e}");
            }
        }
        println!("finished trace_provider flush");
        // tokio::time::sleep(Duration::from_secs(10)).await;
    })
    .await;


    tokio::time::sleep(Duration::from_secs(10)).await;
}

async fn foo() -> usize {
    let mut sum = 0;
    for _ in 0..10 {
        sum += bar().await
    }
    sum
}

async fn bar() -> usize {
    tokio::time::sleep(Duration::from_millis(100)).await;
    42
}
