use crate::export::trace::SpanData;
use crate::trace::{Config, EvictedHashMap, EvictedQueue};
use opentelemetry_api::trace::{
    SpanContext, SpanId, SpanKind, Status, TraceFlags, TraceId, TraceState,
};
use opentelemetry_api::InstrumentationLibrary;

mod span_exporter;

pub use opentelemetry_api::testing::trace::TestSpan;
pub use span_exporter::{NoopSpanExporter, TestExportError, TestSpanExporter, TokioSpanExporter};

pub fn new_test_export_span_data() -> SpanData {
    let config = Config::default();
    SpanData {
        span_context: SpanContext::new(
            TraceId::from_u128(1),
            SpanId::from_u64(1),
            TraceFlags::SAMPLED,
            false,
            TraceState::default(),
        ),
        parent_span_id: SpanId::INVALID,
        span_kind: SpanKind::Internal,
        name: "opentelemetry".into(),
        start_time: opentelemetry_api::time::now(),
        end_time: opentelemetry_api::time::now(),
        attributes: EvictedHashMap::new(config.span_limits.max_attributes_per_span, 0),
        events: EvictedQueue::new(config.span_limits.max_events_per_span),
        links: EvictedQueue::new(config.span_limits.max_links_per_span),
        status: Status::Unset,
        resource: config.resource,
        instrumentation_lib: InstrumentationLibrary::default(),
    }
}
