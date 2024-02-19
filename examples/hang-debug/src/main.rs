// src/main.rs
use opentelemetry::trace::{FutureExt, SpanKind, TraceContextExt, Tracer, TracerProvider};
use opentelemetry::{global, Context, KeyValue};
use opentelemetry_sdk as sdk;
use opentelemetry_sdk::export::trace::SpanData;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::testing::trace::NoopSpanExporter;
use opentelemetry_sdk::trace::{BatchConfigBuilder, SpanProcessor};
use opentelemetry_sdk::{
    trace::{RandomIdGenerator, Sampler},
    Resource,
};
use std::time::Duration;
use tokio::task::yield_now;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    console_subscriber::init();

    let mut provider_builder = sdk::trace::TracerProvider::builder();
    let mut batch_processor =
        sdk::trace::BatchSpanProcessor::builder(NoopSpanExporter::new(), Tokio)
            .with_batch_config(
                BatchConfigBuilder::default()
                    // .with_max_concurrent_exports(1)
                    // shorten the delay between when spans are flushed, default is 5s,
                    .with_scheduled_delay(Duration::from_secs(2))
                    .build(),
            )
            .build();

    // batch_processor.force_flush().unwrap();
    // tokio::time::sleep(Duration::from_secs(10)).await;
    provider_builder = provider_builder.with_span_processor(batch_processor);

    provider_builder = provider_builder.with_config(
        opentelemetry_sdk::trace::config()
            .with_sampler(Sampler::AlwaysOn)
            .with_id_generator(RandomIdGenerator::default())
            .with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "FooService",
            )])),
    );
    let provider = provider_builder.build();
    let tracer = provider.versioned_tracer(
        "opentelemetry-otlp",
        Some(env!("CARGO_PKG_VERSION")),
        Some("https://opentelemetry.io"),
        None,
    );
    let _ = global::set_tracer_provider(provider);

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


        println!("start trace_provider flush");
        let _ =  tracer_provider.force_push_async().await;
        println!("finished trace_provider flush");
    }).await;

    // handle.await.expect("force_flush");

    println!("dropping tracer_provider")
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
