//! A set of span exporters for testing purposes.
use crate::export::trace::{ExportResult, SpanData, SpanExporter};
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use opentelemetry_api::ExportError;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::{channel, Receiver, Sender};

/// A no-op instance of an [`SpanExporter`].
///
/// It will return immediately with success when `export` is called.
///
/// [`SpanExporter`]: SpanExporter
#[derive(Debug, Default)]
pub struct NoopSpanExporter {
    _private: (),
}

impl NoopSpanExporter {
    /// Create a new noop span exporter
    pub fn new() -> Self {
        NoopSpanExporter { _private: () }
    }
}

#[async_trait::async_trait]
impl SpanExporter for NoopSpanExporter {
    fn export(&mut self, _: Vec<SpanData>) -> BoxFuture<'static, ExportResult> {
        Box::pin(std::future::ready(Ok(())))
    }
}

/// A test span exporter that send all the spans to callers.
#[derive(Debug)]
pub struct TestSpanExporter {
    tx_export: Sender<SpanData>,
    tx_shutdown: Sender<()>,
}

#[async_trait]
impl SpanExporter for TestSpanExporter {
    fn export(&mut self, batch: Vec<SpanData>) -> BoxFuture<'static, ExportResult> {
        for span_data in batch {
            if let Err(err) = self
                .tx_export
                .send(span_data)
                .map_err::<TestExportError, _>(Into::into)
            {
                return Box::pin(std::future::ready(Err(Into::into(err))));
            }
        }
        Box::pin(std::future::ready(Ok(())))
    }

    fn shutdown(&mut self) {
        let _ = self.tx_shutdown.send(()); // ignore error
    }
}

impl TestSpanExporter {
    /// create a test span exporter that will send all span data to the receiver returned
    ///
    pub fn new() -> (TestSpanExporter, Receiver<SpanData>, Receiver<()>) {
        let (tx_export, rx_export) = channel();
        let (tx_shutdown, rx_shutdown) = channel();
        let exporter = TestSpanExporter {
            tx_export,
            tx_shutdown,
        };
        (exporter, rx_export, rx_shutdown)
    }
}

/// A test span exporter that send all the spans to callers using tokio mpsc channels.
#[derive(Debug)]
pub struct TokioSpanExporter {
    tx_export: tokio::sync::mpsc::UnboundedSender<SpanData>,
    tx_shutdown: tokio::sync::mpsc::UnboundedSender<()>,
}

impl SpanExporter for TokioSpanExporter {
    fn export(&mut self, batch: Vec<SpanData>) -> BoxFuture<'static, ExportResult> {
        for span_data in batch {
            if let Err(err) = self
                .tx_export
                .send(span_data)
                .map_err::<TestExportError, _>(Into::into)
            {
                return Box::pin(std::future::ready(Err(Into::into(err))));
            }
        }
        Box::pin(std::future::ready(Ok(())))
    }

    fn shutdown(&mut self) {
        self.tx_shutdown.send(()).unwrap();
    }
}

impl TokioSpanExporter {
    pub fn new() -> (
        TokioSpanExporter,
        tokio::sync::mpsc::UnboundedReceiver<SpanData>,
        tokio::sync::mpsc::UnboundedReceiver<()>,
    ) {
        let (tx_export, rx_export) = tokio::sync::mpsc::unbounded_channel();
        let (tx_shutdown, rx_shutdown) = tokio::sync::mpsc::unbounded_channel();
        let exporter = TokioSpanExporter {
            tx_export,
            tx_shutdown,
        };
        (exporter, rx_export, rx_shutdown)
    }
}

#[derive(Debug)]
pub struct TestExportError(String);

impl std::error::Error for TestExportError {}

impl ExportError for TestExportError {
    fn exporter_name(&self) -> &'static str {
        "test"
    }
}

impl Display for TestExportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for TestExportError {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
        TestExportError(err.to_string())
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for TestExportError {
    fn from(err: std::sync::mpsc::SendError<T>) -> Self {
        TestExportError(err.to_string())
    }
}
