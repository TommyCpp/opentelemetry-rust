use std::sync::Arc;

use opentelemetry_api::metrics::{
    AsyncCounter, AsyncGauge, AsyncUpDownCounter, Counter, Histogram, InstrumentProvider, Meter,
    ObservableCounter, ObservableGauge, ObservableUpDownCounter, Result, SyncCounter,
    SyncHistogram, SyncUpDownCounter, Unit, UpDownCounter,
};
use opentelemetry_api::{Context, InstrumentationLibrary, KeyValue};

use crate::metrics::aggregators::AggregatorBuilder;
use crate::metrics::sdk_api::{
    AsyncInstrumentCore, Descriptor, InstrumentKind, MeterCore, Number, NumberKind,
    SyncInstrumentCore,
};
use crate::metrics::view::View;

/// wraps impl to be a full implementation of a Meter.
///
/// Note that the `views` should only contains view that matches the meter's name, version and schema url(if applicable).
pub fn wrap_meter_core(
    core: Arc<dyn MeterCore + Send + Sync>,
    library: InstrumentationLibrary,
    filtered_views: Vec<View>,
) -> Meter {
    Meter::new(library, Arc::new(MeterImpl::new(core, filtered_views)))
}

struct MeterImpl {
    core: Arc<dyn MeterCore + Send + Sync>,
    views: Vec<View>, // sort by the selector's specificity from high to low
}

impl MeterImpl {
    fn new(core: Arc<dyn MeterCore + Send + Sync>, mut views: Vec<View>) -> Self {
        views.sort_by(|a, b| a.selector.selector_num().cmp(&b.selector.selector_num()));
        MeterImpl { core, views }
    }

    fn select_views(
        &self,
        instrument_name: &String,
        instrument_unit: &Option<Unit>,
        instrument_kind: InstrumentKind,
    ) -> Arc<dyn AggregatorBuilder> {
        // note that the view is already sorted by the selector's specificity.
        let candidates = self
            .views
            .iter()
            .filter(|view| {
                // we don't need to check meter name, version and schema url here because the views are already filtered
                let selector = &view.selector;
                if selector.instrument_name != "*" && selector.instrument_name != *instrument_name {
                    return false;
                }
                if let (Some(unit), Some(target_unit)) =
                    (&selector.instrument_unit, instrument_unit)
                {
                    if *unit != *target_unit {
                        return false;
                    }
                }
                if let Some(kind) = &selector.instrument_kind {
                    if *kind != instrument_kind {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<&View>>();
        if candidates.is_empty() {
            return instrument_kind.default_aggregator_builder();
        }
        return candidates
            .first()
            .and_then(|view| view.aggregation_builder())
            .unwrap_or_else(|| instrument_kind.default_aggregator_builder());
    }
}

impl InstrumentProvider for MeterImpl {
    fn u64_counter(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<Counter<u64>> {
        let aggregator_builder = self.select_views(&name, &unit, InstrumentKind::Counter);
        let instrument = self.core.new_sync_instrument(
            Descriptor::new(
                name,
                InstrumentKind::Counter,
                NumberKind::U64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(Counter::new(Arc::new(SyncInstrument(instrument))))
    }

    fn f64_counter(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<Counter<f64>> {
        let aggregator_builder = self.select_views(&name, &unit, InstrumentKind::Counter);
        let instrument = self.core.new_sync_instrument(
            Descriptor::new(
                name,
                InstrumentKind::Counter,
                NumberKind::F64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(Counter::new(Arc::new(SyncInstrument(instrument))))
    }

    fn u64_observable_counter(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<ObservableCounter<u64>> {
        let aggregator_builder = self.select_views(&name, &unit, InstrumentKind::CounterObserver);
        let instrument = self.core.new_async_instrument(
            Descriptor::new(
                name,
                InstrumentKind::CounterObserver,
                NumberKind::U64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(ObservableCounter::new(Arc::new(AsyncInstrument(
            instrument,
        ))))
    }

    fn f64_observable_counter(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<ObservableCounter<f64>> {
        let aggregator_builder = self.select_views(&name, &unit, InstrumentKind::CounterObserver);
        let instrument = self.core.new_async_instrument(
            Descriptor::new(
                name,
                InstrumentKind::CounterObserver,
                NumberKind::F64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(ObservableCounter::new(Arc::new(AsyncInstrument(
            instrument,
        ))))
    }

    fn i64_up_down_counter(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<UpDownCounter<i64>> {
        let aggregator_builder = self.select_views(&name, &unit, InstrumentKind::UpDownCounter);
        let instrument = self.core.new_sync_instrument(
            Descriptor::new(
                name,
                InstrumentKind::UpDownCounter,
                NumberKind::I64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(UpDownCounter::new(Arc::new(SyncInstrument(instrument))))
    }

    fn f64_up_down_counter(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<UpDownCounter<f64>> {
        let aggregator_builder = self.select_views(&name, &unit, InstrumentKind::UpDownCounter);
        let instrument = self.core.new_sync_instrument(
            Descriptor::new(
                name,
                InstrumentKind::UpDownCounter,
                NumberKind::F64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(UpDownCounter::new(Arc::new(SyncInstrument(instrument))))
    }

    fn i64_observable_up_down_counter(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<ObservableUpDownCounter<i64>> {
        let aggregator_builder =
            self.select_views(&name, &unit, InstrumentKind::UpDownCounterObserver);
        let instrument = self.core.new_async_instrument(
            Descriptor::new(
                name,
                InstrumentKind::UpDownCounterObserver,
                NumberKind::I64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(ObservableUpDownCounter::new(Arc::new(AsyncInstrument(
            instrument,
        ))))
    }

    fn f64_observable_up_down_counter(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<ObservableUpDownCounter<f64>> {
        let aggregator_builder =
            self.select_views(&name, &unit, InstrumentKind::UpDownCounterObserver);
        let instrument = self.core.new_async_instrument(
            Descriptor::new(
                name,
                InstrumentKind::UpDownCounterObserver,
                NumberKind::F64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(ObservableUpDownCounter::new(Arc::new(AsyncInstrument(
            instrument,
        ))))
    }

    fn u64_observable_gauge(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<ObservableGauge<u64>> {
        let aggregator_builder = self.select_views(&name, &unit, InstrumentKind::GaugeObserver);
        let instrument = self.core.new_async_instrument(
            Descriptor::new(
                name,
                InstrumentKind::GaugeObserver,
                NumberKind::U64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(ObservableGauge::new(Arc::new(AsyncInstrument(instrument))))
    }

    fn i64_observable_gauge(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<ObservableGauge<i64>> {
        let aggregator_builder = self.select_views(&name, &unit, InstrumentKind::GaugeObserver);
        let instrument = self.core.new_async_instrument(
            Descriptor::new(
                name,
                InstrumentKind::GaugeObserver,
                NumberKind::I64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(ObservableGauge::new(Arc::new(AsyncInstrument(instrument))))
    }

    fn f64_observable_gauge(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<ObservableGauge<f64>> {
        let aggregator_builder = self.select_views(&name, &unit, InstrumentKind::GaugeObserver);
        let instrument = self.core.new_async_instrument(
            Descriptor::new(
                name,
                InstrumentKind::GaugeObserver,
                NumberKind::F64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(ObservableGauge::new(Arc::new(AsyncInstrument(instrument))))
    }

    fn f64_histogram(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<Histogram<f64>> {
        let aggregator_builder = self.select_views(&name, &unit, InstrumentKind::Histogram);
        let instrument = self.core.new_sync_instrument(
            Descriptor::new(
                name,
                InstrumentKind::Histogram,
                NumberKind::F64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(Histogram::new(Arc::new(SyncInstrument(instrument))))
    }

    fn u64_histogram(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<Histogram<u64>> {
        let aggregator_builder = self.select_views(&name, &unit, InstrumentKind::Histogram);
        let instrument = self.core.new_sync_instrument(
            Descriptor::new(
                name,
                InstrumentKind::Histogram,
                NumberKind::U64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(Histogram::new(Arc::new(SyncInstrument(instrument))))
    }

    fn i64_histogram(
        &self,
        name: String,
        description: Option<String>,
        unit: Option<Unit>,
    ) -> Result<Histogram<i64>> {
        let aggregator_builder = self.select_views(&name, &unit, InstrumentKind::Histogram);
        let instrument = self.core.new_sync_instrument(
            Descriptor::new(
                name,
                InstrumentKind::Histogram,
                NumberKind::I64,
                description,
                unit,
            ),
            aggregator_builder,
        )?;

        Ok(Histogram::new(Arc::new(SyncInstrument(instrument))))
    }

    fn register_callback(&self, callback: Box<dyn Fn(&Context) + Send + Sync>) -> Result<()> {
        self.core.register_callback(callback)
    }
}

struct SyncInstrument(Arc<dyn SyncInstrumentCore + Send + Sync>);

impl<T: Into<Number>> SyncCounter<T> for SyncInstrument {
    fn add(&self, cx: &Context, value: T, attributes: &[KeyValue]) {
        self.0.record_one(cx, value.into(), attributes)
    }
}

impl<T: Into<Number>> SyncUpDownCounter<T> for SyncInstrument {
    fn add(&self, cx: &Context, value: T, attributes: &[KeyValue]) {
        self.0.record_one(cx, value.into(), attributes)
    }
}

impl<T: Into<Number>> SyncHistogram<T> for SyncInstrument {
    fn record(&self, cx: &Context, value: T, attributes: &[KeyValue]) {
        self.0.record_one(cx, value.into(), attributes)
    }
}

struct AsyncInstrument(Arc<dyn AsyncInstrumentCore + Send + Sync>);

impl<T: Into<Number>> AsyncCounter<T> for AsyncInstrument {
    fn observe(&self, cx: &Context, value: T, attributes: &[KeyValue]) {
        self.0.observe_one(cx, value.into(), attributes)
    }
}

impl<T: Into<Number>> AsyncUpDownCounter<T> for AsyncInstrument {
    fn observe(&self, cx: &Context, value: T, attributes: &[KeyValue]) {
        self.0.observe_one(cx, value.into(), attributes)
    }
}

impl<T: Into<Number>> AsyncGauge<T> for AsyncInstrument {
    fn observe(&self, cx: &Context, value: T, attributes: &[KeyValue]) {
        self.0.observe_one(cx, value.into(), attributes)
    }
}
