use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use opentelemetry_api::InstrumentationLibrary;
use opentelemetry_api::metrics::Meter_OLD;
use crate::metrics::registry::UniqueInstrumentMeterCore;

pub struct MeterProvider {
    meters: Mutex<HashMap<InstrumentationLibrary, Arc<UniqueInstrumentMeterCore>>>
}

impl opentelemetry_api::metrics::MeterProvider for MeterProvider {
    fn versioned_meter(&self, name: &'static str, version: Option<&'static str>, schema_url: Option<&'static str>) -> Meter_OLD {
        self.meters.lock().map(|map| {
            map.
        })
    }
}