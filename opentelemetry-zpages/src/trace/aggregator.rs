//! ## Span Aggregator
//!
//! Process the span information, aggregate counts for latency, running, and errors for spans grouped
//! by name.
use futures::channel::mpsc;
use crate::trace::span_processor::TracezMessage;
use opentelemetry::exporter::trace::SpanData;
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use opentelemetry::trace::SpanId;
use futures::{FutureExt, StreamExt, Stream};
use futures::task::Poll;
use std::borrow::BorrowMut;

lazy_static! {
    static ref LATENCY_BUCKET: [Duration;9] = [
        Duration::from_micros(0),
        Duration::from_micros(10),
        Duration::from_micros(100),
        Duration::from_millis(1),
        Duration::from_millis(10),
        Duration::from_millis(100),
        Duration::from_secs(1),
        Duration::from_secs(10),
        Duration::from_secs(100),
    ];
}
const LATENCY_BUCKET_COUNT: usize = 9;

/// Aggregate span information from zPage span processor and feed that information to server when
/// requested.
pub struct SpanAggregator {
    receiver: mpsc::Receiver<TracezMessage>,
    summaries: HashMap<String, SpanSummary>,
}

impl SpanAggregator {
    /// Create a span aggregator
    pub fn new(receiver: mpsc::Receiver<TracezMessage>) -> SpanAggregator {
        SpanAggregator {
            receiver,
            summaries: HashMap::new(),
        }
    }

    /// Process request from http server or the span processor.
    pub async fn process(&mut self) {
        loop {
            match self.receiver.try_next() {
                Err(_) => {
                    unimplemented!();
                }
                Ok(None) => {
                    // all senders have been dropped. Thus, close it
                    self.receiver.close();
                    return;
                }
                Ok(Some(msg)) => {
                    match msg {
                        TracezMessage::ShutDown => {
                            self.receiver.close();
                            return;
                        }
                        TracezMessage::SampleSpan(span) => {
                            let summary = self.summaries.entry(span.name.clone()).or_default();
                            summary.running_sample_span = Some(span);
                            summary.running_num += 1;
                        }
                        TracezMessage::SpanStart {
                            span_name, ..
                        } => {
                            let summary = self.summaries.entry(span_name).or_default();
                            summary.running_num += 1;
                        }
                        _ => {
                            unimplemented!()
                        }
                    }
                }
            }
        }
    }
}

struct SpanSummary {
    pub running_sample_span: Option<SpanData>,
    pub error_sample_span: Option<SpanData>,
    pub latency_sample_span: [Option<SpanData>; LATENCY_BUCKET_COUNT],

    pub latency: [usize; 9],
    pub error_num: usize,
    pub running_num: usize,
}

impl Default for SpanSummary {
    fn default() -> Self {
        SpanSummary {
            running_sample_span: None,
            error_sample_span: None,
            latency_sample_span: Default::default(),
            latency: Default::default(),
            error_num: 0,
            running_num: 0,
        }
    }
}

impl SpanSummary {
    /// Create a span summary with given span as running example
    pub fn with_running_sample(span: SpanData) -> SpanSummary {
        SpanSummary {
            running_sample_span: Some(span),
            ..SpanSummary::default()
        }
    }
}