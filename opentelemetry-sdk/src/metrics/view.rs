//! A `View` provides SDK users with the flexibility to customize the metrics that are output by the
//! SDK. Here are some examples when a View might be needed:
//!
//! - Customize which Instruments are to be processed/ignored. For example, an instrumented library
//! can provide both temperature and humidity, but the application developer might only want temperature.
//!
//! - Customize the aggregation - if the default aggregation associated with the Instrument does not
//! meet the needs of the user. For example, an HTTP client library might expose HTTP client request
//! duration as Histogram by default, but the application developer might only want the total count
//! of outgoing requests.
//!
//! - Customize which attribute(s) are to be reported on metrics. For example, an HTTP server library
//! might expose HTTP verb (e.g. GET, POST) and HTTP status code (e.g. 200, 301, 404). The application
//! developer might only care about HTTP status code (e.g. reporting the total count of HTTP requests
//! for each HTTP status code). There could also be extreme scenarios in which the application developer
//! does not need any attributes (e.g. just get the total count of all incoming requests).
//!
//!
//! ## Named view
//! Since `View` name is used as metrics stream name, it means if a named view select multiple instruments,
//! the output will contains multiple metrics stream with same name. Thus, the instrument selector of a named view
//! should match only one instrument.
//!
//! For view without a name, SDK will

use crate::metrics::aggregators::AggregatorBuilder;
use crate::metrics::sdk_api::InstrumentKind;
use opentelemetry_api::metrics::Unit;
use opentelemetry_api::Key;
use std::sync::Arc;

/// Select instruments by name, kind, unit or meter name, version and schema_url.
///
/// Note that only the instrument that meets **all** condition will be selected.
///
/// ## Wildcard
/// Wildcard like `*` and `?` are not supported at the moment but maybe added in the future.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InstrumentSelector {
    pub(crate) instrument_kind: Option<InstrumentKind>,
    pub(crate) instrument_name: String,
    pub(crate) instrument_unit: Option<Unit>,
    pub(crate) meter_name: Option<String>,
    pub(crate) meter_version: Option<String>,
    pub(crate) meter_schema_url: Option<String>,
}

impl Default for InstrumentSelector {
    fn default() -> Self {
        Self {
            instrument_kind: None,
            instrument_name: "*".to_string(),
            instrument_unit: None,
            meter_name: None,
            meter_version: None,
            meter_schema_url: None,
        }
    }
}

impl InstrumentSelector {
    /// Select instruments by its name.
    ///
    /// Wildcard are not supported in general. But `*` is supported as a special case to match all instruments regardless of their names.
    pub fn instrument_name<T: Into<String>>(instrument_name: T) -> Self {
        Self {
            instrument_name: instrument_name.into(),
            ..Self::default()
        }
    }

    /// Select instruments by its kind. See [`InstrumentKind`] for more information.
    pub fn with_instrument_kind(mut self, instrument_kind: InstrumentKind) -> Self {
        self.instrument_kind = Some(instrument_kind);
        self
    }

    /// Select instruments by its unit.
    pub fn with_instrument_unit(mut self, instrument_unit: Unit) -> Self {
        self.instrument_unit = Some(instrument_unit);
        self
    }

    /// Select instruments by its meter name.
    pub fn with_meter_name(mut self, meter_name: String) -> Self {
        self.meter_name = Some(meter_name);
        self
    }

    /// Select instruments by its meter version.
    pub fn with_meter_version(mut self, meter_version: String) -> Self {
        self.meter_version = Some(meter_version);
        self
    }

    /// Select instruments by its meter schema url.
    pub fn with_meter_schema_url(mut self, meter_schema_url: String) -> Self {
        self.meter_schema_url = Some(meter_schema_url);
        self
    }

    // todo: add support to set InstrumentScope/InstrumentLibrary
}

/// Build a new `View`. See module level doc for more details.
///
/// User MUST provide at least one criteria.
///
// view will carry an aggregation builder, which will be called when we adding a new Record to build
// an aggregator for that record
#[derive(Debug, Clone)]
pub struct View {
    pub(crate) view_name: Option<String>,
    pub(crate) selector: InstrumentSelector,

    // configuration for metrics stream below
    pub(crate) metrics_stream_desc: Option<String>,
    pub(crate) exported_attribute_keys: Vec<Key>,
    pub(crate) aggregation_builder: Option<Arc<dyn AggregatorBuilder>>,
}

impl View {
    /// The selector of instrument that this view is applied to, required.
    ///
    pub fn new(selector: InstrumentSelector) -> Self {
        return View {
            view_name: None,
            metrics_stream_desc: None,
            exported_attribute_keys: vec![],
            aggregation_builder: None,
            selector,
        }
    }

    /// The name of the View, optional. This will be used as the name of metric stream.
    ///
    /// If not provided, the instrument name will be used by default as the metric stream name.
    // todo: named view should only matches one instrument
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.view_name = Some(name.into());
        self
    }

    /// The description of the metric stream, optional.
    ///
    /// If not provided, the instrument description will be used by default.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.metrics_stream_desc = Some(description.into());
        self
    }

    /// The attributes should be exported in the metric stream, optional.
    ///
    /// If not provided, all attribute keys will be used.
    pub fn with_exported_attributes(mut self, keys: Vec<Key>) -> Self {
        self.exported_attribute_keys = keys;
        self
    }

    /// The aggregation that should be applied to the instrument, optional.
    ///
    /// If not provided, the SDK will apply a default aggregation based on the instrument type.
    pub fn with_aggregation<Agg>(mut self, aggregation_builder: Agg) -> Self
    where
        Agg: AggregatorBuilder,
    {
        self.aggregation_builder = Some(Arc::new(aggregation_builder));
        self
    }

    /// Return a copy of aggregation builder.
    pub fn aggregation_builder(&self) -> Option<Arc<dyn AggregatorBuilder>> {
        self.aggregation_builder.clone()
    }
}
