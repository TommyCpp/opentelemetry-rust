//! # No-op OpenTelemetry Metrics Implementation
//!
//! This implementation is returned as the global Meter if no `Meter`
//! has been set. It is also useful for testing purposes as it is intended
//! to have minimal resource utilization and runtime impact.
use crate::{
    metrics::{
        AsyncCounter, AsyncGauge, AsyncUpDownCounter, InstrumentProvider, MeterProvider, Meter_OLD,
        Result, SyncCounter, SyncHistogram, SyncUpDownCounter,
    },
    Context, InstrumentationLibrary, KeyValue,
};
use std::sync::Arc;
use crate::metrics::{Counter, Histogram, InstrumentBuilder, MetricsError, ObservableCounter, ObservableGauge, ObservableUpDownCounter, UpDownCounter};
use crate::metrics::meter::Meter;

/// A no-op instance of a `MetricProvider`
#[derive(Debug, Default)]
pub struct NoopMeterProvider {
    _private: (),
}

impl NoopMeterProvider {
    /// Create a new no-op meter provider.
    pub fn new() -> Self {
        NoopMeterProvider { _private: () }
    }
}

impl MeterProvider for NoopMeterProvider {
    type Meter = NoopMeter;

    fn versioned_meter(
        &self,
        name: &'static str,
        version: Option<&'static str>,
        schema_url: Option<&'static str>,
    ) -> Meter_OLD {
        let library = InstrumentationLibrary::new(name, version, schema_url);
        Meter_OLD::new(library, Arc::new(NoopMeterCore::new()))
    }
}

/// A no-op instance of a `Meter`
#[derive(Debug, Default)]
pub struct NoopMeterCore {
    _private: (),
}

impl NoopMeterCore {
    /// Create a new no-op meter core.
    pub fn new() -> Self {
        NoopMeterCore { _private: () }
    }
}

impl InstrumentProvider for NoopMeterCore {
    fn register_callback(&self, _callback: Box<dyn Fn(&Context) + Send + Sync>) -> Result<()> {
        Ok(())
    }
}

/// A no-op sync instrument
#[derive(Debug, Default)]
pub struct NoopSyncInstrument {
    _private: (),
}

impl NoopSyncInstrument {
    /// Create a new no-op sync instrument
    pub fn new() -> Self {
        NoopSyncInstrument { _private: () }
    }
}

impl<T> SyncCounter<T> for NoopSyncInstrument {
    fn add(&self, _cx: &Context, _value: T, _attributes: &[KeyValue]) {
        // Ignored
    }
}

impl<T> SyncUpDownCounter<T> for NoopSyncInstrument {
    fn add(&self, _cx: &Context, _value: T, _attributes: &[KeyValue]) {
        // Ignored
    }
}

impl<T> SyncHistogram<T> for NoopSyncInstrument {
    fn record(&self, _cx: &Context, _value: T, _attributes: &[KeyValue]) {
        // Ignored
    }
}

/// A no-op async instrument.
#[derive(Debug, Default)]
pub struct NoopAsyncInstrument {
    _private: (),
}

impl NoopAsyncInstrument {
    /// Create a new no-op async instrument
    pub fn new() -> Self {
        NoopAsyncInstrument { _private: () }
    }
}

impl<T> AsyncGauge<T> for NoopAsyncInstrument {
    fn observe(&self, _cx: &Context, _value: T, _attributes: &[KeyValue]) {
        // Ignored
    }
}

impl<T> AsyncCounter<T> for NoopAsyncInstrument {
    fn observe(&self, _cx: &Context, _value: T, _attributes: &[KeyValue]) {
        // Ignored
    }
}

impl<T> AsyncUpDownCounter<T> for NoopAsyncInstrument {
    fn observe(&self, _cx: &Context, _value: T, _attributes: &[KeyValue]) {
        // Ignored
    }
}

pub struct NoopMeter {
    _private: (),
}

impl Meter for NoopMeter {
    fn u64_counter(&self, name: impl Into<String>) -> InstrumentBuilder<'_, Counter<u64>> {
        todo!()
    }

    fn f64_counter(&self, name: impl Into<String>) -> InstrumentBuilder<'_, Counter<f64>> {
        todo!()
    }

    fn u64_observable_counter(&self, name: impl Into<String>) -> InstrumentBuilder<'_, ObservableCounter<u64>> {
        todo!()
    }

    fn f64_observable_counter(&self, name: impl Into<String>) -> InstrumentBuilder<'_, ObservableCounter<f64>> {
        todo!()
    }

    fn i64_up_down_counter(&self, name: impl Into<String>) -> InstrumentBuilder<'_, UpDownCounter<i64>> {
        todo!()
    }

    fn f64_up_down_counter(&self, name: impl Into<String>) -> InstrumentBuilder<'_, UpDownCounter<f64>> {
        todo!()
    }

    fn i64_observable_up_down_counter(&self, name: impl Into<String>) -> InstrumentBuilder<'_, ObservableUpDownCounter<i64>> {
        todo!()
    }

    fn f64_observable_up_down_counter(&self, name: impl Into<String>) -> InstrumentBuilder<'_, ObservableUpDownCounter<f64>> {
        todo!()
    }

    fn u64_observable_gauge(&self, name: impl Into<String>) -> InstrumentBuilder<'_, ObservableGauge<u64>> {
        todo!()
    }

    fn i64_observable_gauge(&self, name: impl Into<String>) -> InstrumentBuilder<'_, ObservableGauge<i64>> {
        todo!()
    }

    fn f64_observable_gauge(&self, name: impl Into<String>) -> InstrumentBuilder<'_, ObservableGauge<f64>> {
        todo!()
    }

    fn f64_histogram(&self, name: impl Into<String>) -> InstrumentBuilder<'_, Histogram<f64>> {
        todo!()
    }

    fn u64_histogram(&self, name: impl Into<String>) -> InstrumentBuilder<'_, Histogram<u64>> {
        todo!()
    }

    fn i64_histogram(&self, name: impl Into<String>) -> InstrumentBuilder<'_, Histogram<i64>> {
        todo!()
    }

    fn register_callback<F>(&self, callback: F) -> std::result::Result<(), MetricsError> where F: Fn(&Context) + Send + Sync + 'static {
        todo!()
    }
}