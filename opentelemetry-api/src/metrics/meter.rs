use crate::metrics::{
    Counter, Histogram, InstrumentBuilder, MetricsError, ObservableCounter, ObservableGauge,
    ObservableUpDownCounter, UpDownCounter,
};
use crate::Context;

/// Returns named meter instances
pub trait MeterProvider {
    type Meter: Meter;
    /// Creates a named [`Meter_OLD`] instance.
    fn meter(&self, name: &'static str) -> Self::Meter {
        self.versioned_meter(name, None, None)
    }

    /// Creates an implementation of the [`Meter_OLD`] interface.
    ///
    /// The instrumentation name must be the name of the library providing instrumentation. This
    /// name may be the same as the instrumented code only if that code provides built-in
    /// instrumentation. If the instrumentation name is empty, then a implementation defined
    /// default name will be used instead.
    fn versioned_meter(
        &self,
        name: &'static str,
        version: Option<&'static str>,
        schema_url: Option<&'static str>,
    ) -> Self::Meter;
}

/// `Meter` provides access to instrument instances for recording metrics.
pub trait Meter {
    fn u64_counter(&self, name: impl Into<String>) -> InstrumentBuilder<'_, Counter<u64>>;

    /// creates an instrument builder for recording increasing values.
    fn f64_counter(&self, name: impl Into<String>) -> InstrumentBuilder<'_, Counter<f64>>;

    /// creates an instrument builder for recording increasing values via callback.
    fn u64_observable_counter(
        &self,
        name: impl Into<String>,
    ) -> InstrumentBuilder<'_, ObservableCounter<u64>>;

    /// creates an instrument builder for recording increasing values via callback.
    fn f64_observable_counter(
        &self,
        name: impl Into<String>,
    ) -> InstrumentBuilder<'_, ObservableCounter<f64>>;

    /// creates an instrument builder for recording changes of a value.
    fn i64_up_down_counter(
        &self,
        name: impl Into<String>,
    ) -> InstrumentBuilder<'_, UpDownCounter<i64>>;

    /// creates an instrument builder for recording changes of a value.
    fn f64_up_down_counter(
        &self,
        name: impl Into<String>,
    ) -> InstrumentBuilder<'_, UpDownCounter<f64>>;

    /// creates an instrument builder for recording changes of a value via callback.
    fn i64_observable_up_down_counter(
        &self,
        name: impl Into<String>,
    ) -> InstrumentBuilder<'_, ObservableUpDownCounter<i64>>;

    /// creates an instrument builder for recording changes of a value via callback.
    fn f64_observable_up_down_counter(
        &self,
        name: impl Into<String>,
    ) -> InstrumentBuilder<'_, ObservableUpDownCounter<f64>>;

    /// creates an instrument builder for recording the current value via callback.
    fn u64_observable_gauge(
        &self,
        name: impl Into<String>,
    ) -> InstrumentBuilder<'_, ObservableGauge<u64>>;

    /// creates an instrument builder for recording the current value via callback.
    fn i64_observable_gauge(
        &self,
        name: impl Into<String>,
    ) -> InstrumentBuilder<'_, ObservableGauge<i64>>;

    /// creates an instrument builder for recording the current value via callback.
    fn f64_observable_gauge(
        &self,
        name: impl Into<String>,
    ) -> InstrumentBuilder<'_, ObservableGauge<f64>>;

    /// creates an instrument builder for recording a distribution of values.
    fn f64_histogram(&self, name: impl Into<String>) -> InstrumentBuilder<'_, Histogram<f64>>;

    /// creates an instrument builder for recording a distribution of values.
    fn u64_histogram(&self, name: impl Into<String>) -> InstrumentBuilder<'_, Histogram<u64>>;

    /// creates an instrument builder for recording a distribution of values.
    fn i64_histogram(&self, name: impl Into<String>) -> InstrumentBuilder<'_, Histogram<i64>>;

    /// Captures the function that will be called during data collection.
    ///
    /// It is only valid to call `observe` within the scope of the passed function.
    fn register_callback<F>(&self, callback: F) -> Result<(), MetricsError>
    where
        F: Fn(&Context) + Send + Sync + 'static;
}
