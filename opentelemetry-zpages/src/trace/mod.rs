//! Tracez implementation
//!
mod aggregator;
mod span_processor;

pub use aggregator::SpanAggregator;
pub use span_processor::ZPagesProcessor;

use opentelemetry::sdk::export::trace::SpanData;

/// Message that used to pass commend between web servers, aggregators and span processors.
#[derive(Debug)]
pub enum TracezMessage {
    /// Sample span on start
    SampleSpan(SpanData),
    /// Span ended
    SpanEnd(SpanData),
    /// Shut down the aggregator
    ShutDown,
}