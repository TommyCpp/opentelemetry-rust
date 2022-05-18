use criterion::{Criterion, criterion_group, criterion_main};

use opentelemetry_api::Context;
use opentelemetry_api::trace::{SpanBuilder, TraceContextExt, Tracer, TracerProvider};
use opentelemetry_sdk::trace::{Config, Sampler, TracerProvider as SdkProvider};

fn sampling_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sampling");

    group.bench_function("always_on", |b| {
        let tracer_provider = SdkProvider::builder()
            .with_config(Config::default().with_sampler(Sampler::TraceIdRatioBased(0.5)))
            .build();

        b.iter(|| {
            generate_trace(1200, tracer_provider.tracer("test"));
        });
    });
}

// generate a trace with multiple level spans, forming a span list.
fn generate_trace(level: i32, tracer: opentelemetry_sdk::trace::Tracer) {
    let mut span = tracer.start("test_root");
    let mut ctx = Context::current_with_span(span);
    for idx in 0..level {
        span = tracer.build_with_context(SpanBuilder::from_name(format!("test_{}", idx)), &ctx);
        ctx = Context::current_with_span(span);
    }
}

criterion_group!(benches, sampling_benchmark);
criterion_main!(benches);
