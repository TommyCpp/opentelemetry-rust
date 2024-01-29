/// LogsData represents the logs data that can be stored in a persistent storage,
/// OR can be embedded by other protocols that transfer OTLP logs data but do not
/// implement the OTLP protocol.
///
/// The main difference between this message and collector protocol is that
/// in this message there will not be any "control" or "metadata" specific to
/// OTLP protocol.
///
/// When new fields are added into this message, the OTLP request MUST be updated
/// as well.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogsData {
    /// An array of ResourceLogs.
    /// For data coming from a single resource this array will typically contain
    /// one element. Intermediary nodes that receive data from multiple origins
    /// typically batch the data before forwarding further and in that case this
    /// array will contain multiple elements.
    #[prost(message, repeated, tag = "1")]
    pub resource_logs: ::prost::alloc::vec::Vec<ResourceLogs>,
}
/// A collection of ScopeLogs from a Resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLogs {
    /// The resource for the logs in this message.
    /// If this field is not set then resource info is unknown.
    #[prost(message, optional, tag = "1")]
    pub resource: ::core::option::Option<super::super::resource::v1::Resource>,
    /// A list of ScopeLogs that originate from a resource.
    #[prost(message, repeated, tag = "2")]
    pub scope_logs: ::prost::alloc::vec::Vec<ScopeLogs>,
    /// The Schema URL, if known. This is the identifier of the Schema that the resource data
    /// is recorded in. To learn more about Schema URL see
    /// <https://opentelemetry.io/docs/specs/otel/schemas/#schema-url>
    /// This schema_url applies to the data in the "resource" field. It does not apply
    /// to the data in the "scope_logs" field which have their own schema_url field.
    #[prost(string, tag = "3")]
    pub schema_url: ::prost::alloc::string::String,
}
/// A collection of Logs produced by a Scope.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeLogs {
    /// The instrumentation scope information for the logs in this message.
    /// Semantically when InstrumentationScope isn't set, it is equivalent with
    /// an empty instrumentation scope name (unknown).
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<super::super::common::v1::InstrumentationScope>,
    /// A list of log records.
    #[prost(message, repeated, tag = "2")]
    pub log_records: ::prost::alloc::vec::Vec<LogRecord>,
    /// The Schema URL, if known. This is the identifier of the Schema that the log data
    /// is recorded in. To learn more about Schema URL see
    /// <https://opentelemetry.io/docs/specs/otel/schemas/#schema-url>
    /// This schema_url applies to all logs in the "logs" field.
    #[prost(string, tag = "3")]
    pub schema_url: ::prost::alloc::string::String,
}
/// A log record according to OpenTelemetry Log Data Model:
/// <https://github.com/open-telemetry/oteps/blob/main/text/logs/0097-log-data-model.md>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogRecord {
    /// time_unix_nano is the time when the event occurred.
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    /// Value of 0 indicates unknown or missing timestamp.
    #[prost(fixed64, tag = "1")]
    pub time_unix_nano: u64,
    /// Time when the event was observed by the collection system.
    /// For events that originate in OpenTelemetry (e.g. using OpenTelemetry Logging SDK)
    /// this timestamp is typically set at the generation time and is equal to Timestamp.
    /// For events originating externally and collected by OpenTelemetry (e.g. using
    /// Collector) this is the time when OpenTelemetry's code observed the event measured
    /// by the clock of the OpenTelemetry code. This field MUST be set once the event is
    /// observed by OpenTelemetry.
    ///
    /// For converting OpenTelemetry log data to formats that support only one timestamp or
    /// when receiving OpenTelemetry log data by recipients that support only one timestamp
    /// internally the following logic is recommended:
    ///    - Use time_unix_nano if it is present, otherwise use observed_time_unix_nano.
    ///
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    /// Value of 0 indicates unknown or missing timestamp.
    #[prost(fixed64, tag = "11")]
    pub observed_time_unix_nano: u64,
    /// Numerical value of the severity, normalized to values described in Log Data Model.
    /// \[Optional\].
    #[prost(enumeration = "SeverityNumber", tag = "2")]
    pub severity_number: i32,
    /// The severity text (also known as log level). The original string representation as
    /// it is known at the source. \[Optional\].
    #[prost(string, tag = "3")]
    pub severity_text: ::prost::alloc::string::String,
    /// A value containing the body of the log record. Can be for example a human-readable
    /// string message (including multi-line) describing the event in a free form or it can
    /// be a structured data composed of arrays and maps of other values. \[Optional\].
    #[prost(message, optional, tag = "5")]
    pub body: ::core::option::Option<super::super::common::v1::AnyValue>,
    /// Additional attributes that describe the specific event occurrence. \[Optional\].
    /// Attribute keys MUST be unique (it is not allowed to have more than one
    /// attribute with the same key).
    #[prost(message, repeated, tag = "6")]
    pub attributes: ::prost::alloc::vec::Vec<super::super::common::v1::KeyValue>,
    #[prost(uint32, tag = "7")]
    pub dropped_attributes_count: u32,
    /// Flags, a bit field. 8 least significant bits are the trace flags as
    /// defined in W3C Trace Context specification. 24 most significant bits are reserved
    /// and must be set to 0. Readers must not assume that 24 most significant bits
    /// will be zero and must correctly mask the bits when reading 8-bit trace flag (use
    /// flags & LOG_RECORD_FLAGS_TRACE_FLAGS_MASK). \[Optional\].
    #[prost(fixed32, tag = "8")]
    pub flags: u32,
    /// A unique identifier for a trace. All logs from the same trace share
    /// the same `trace_id`. The ID is a 16-byte array. An ID with all zeroes OR
    /// of length other than 16 bytes is considered invalid (empty string in OTLP/JSON
    /// is zero-length and thus is also invalid).
    ///
    /// This field is optional.
    ///
    /// The receivers SHOULD assume that the log record is not associated with a
    /// trace if any of the following is true:
    ///    - the field is not present,
    ///    - the field contains an invalid value.
    #[prost(bytes = "vec", tag = "9")]
    pub trace_id: ::prost::alloc::vec::Vec<u8>,
    /// A unique identifier for a span within a trace, assigned when the span
    /// is created. The ID is an 8-byte array. An ID with all zeroes OR of length
    /// other than 8 bytes is considered invalid (empty string in OTLP/JSON
    /// is zero-length and thus is also invalid).
    ///
    /// This field is optional. If the sender specifies a valid span_id then it SHOULD also
    /// specify a valid trace_id.
    ///
    /// The receivers SHOULD assume that the log record is not associated with a
    /// span if any of the following is true:
    ///    - the field is not present,
    ///    - the field contains an invalid value.
    #[prost(bytes = "vec", tag = "10")]
    pub span_id: ::prost::alloc::vec::Vec<u8>,
}
/// Possible values for LogRecord.SeverityNumber.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SeverityNumber {
    /// UNSPECIFIED is the default SeverityNumber, it MUST NOT be used.
    Unspecified = 0,
    Trace = 1,
    Trace2 = 2,
    Trace3 = 3,
    Trace4 = 4,
    Debug = 5,
    Debug2 = 6,
    Debug3 = 7,
    Debug4 = 8,
    Info = 9,
    Info2 = 10,
    Info3 = 11,
    Info4 = 12,
    Warn = 13,
    Warn2 = 14,
    Warn3 = 15,
    Warn4 = 16,
    Error = 17,
    Error2 = 18,
    Error3 = 19,
    Error4 = 20,
    Fatal = 21,
    Fatal2 = 22,
    Fatal3 = 23,
    Fatal4 = 24,
}
impl SeverityNumber {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SeverityNumber::Unspecified => "SEVERITY_NUMBER_UNSPECIFIED",
            SeverityNumber::Trace => "SEVERITY_NUMBER_TRACE",
            SeverityNumber::Trace2 => "SEVERITY_NUMBER_TRACE2",
            SeverityNumber::Trace3 => "SEVERITY_NUMBER_TRACE3",
            SeverityNumber::Trace4 => "SEVERITY_NUMBER_TRACE4",
            SeverityNumber::Debug => "SEVERITY_NUMBER_DEBUG",
            SeverityNumber::Debug2 => "SEVERITY_NUMBER_DEBUG2",
            SeverityNumber::Debug3 => "SEVERITY_NUMBER_DEBUG3",
            SeverityNumber::Debug4 => "SEVERITY_NUMBER_DEBUG4",
            SeverityNumber::Info => "SEVERITY_NUMBER_INFO",
            SeverityNumber::Info2 => "SEVERITY_NUMBER_INFO2",
            SeverityNumber::Info3 => "SEVERITY_NUMBER_INFO3",
            SeverityNumber::Info4 => "SEVERITY_NUMBER_INFO4",
            SeverityNumber::Warn => "SEVERITY_NUMBER_WARN",
            SeverityNumber::Warn2 => "SEVERITY_NUMBER_WARN2",
            SeverityNumber::Warn3 => "SEVERITY_NUMBER_WARN3",
            SeverityNumber::Warn4 => "SEVERITY_NUMBER_WARN4",
            SeverityNumber::Error => "SEVERITY_NUMBER_ERROR",
            SeverityNumber::Error2 => "SEVERITY_NUMBER_ERROR2",
            SeverityNumber::Error3 => "SEVERITY_NUMBER_ERROR3",
            SeverityNumber::Error4 => "SEVERITY_NUMBER_ERROR4",
            SeverityNumber::Fatal => "SEVERITY_NUMBER_FATAL",
            SeverityNumber::Fatal2 => "SEVERITY_NUMBER_FATAL2",
            SeverityNumber::Fatal3 => "SEVERITY_NUMBER_FATAL3",
            SeverityNumber::Fatal4 => "SEVERITY_NUMBER_FATAL4",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEVERITY_NUMBER_UNSPECIFIED" => Some(Self::Unspecified),
            "SEVERITY_NUMBER_TRACE" => Some(Self::Trace),
            "SEVERITY_NUMBER_TRACE2" => Some(Self::Trace2),
            "SEVERITY_NUMBER_TRACE3" => Some(Self::Trace3),
            "SEVERITY_NUMBER_TRACE4" => Some(Self::Trace4),
            "SEVERITY_NUMBER_DEBUG" => Some(Self::Debug),
            "SEVERITY_NUMBER_DEBUG2" => Some(Self::Debug2),
            "SEVERITY_NUMBER_DEBUG3" => Some(Self::Debug3),
            "SEVERITY_NUMBER_DEBUG4" => Some(Self::Debug4),
            "SEVERITY_NUMBER_INFO" => Some(Self::Info),
            "SEVERITY_NUMBER_INFO2" => Some(Self::Info2),
            "SEVERITY_NUMBER_INFO3" => Some(Self::Info3),
            "SEVERITY_NUMBER_INFO4" => Some(Self::Info4),
            "SEVERITY_NUMBER_WARN" => Some(Self::Warn),
            "SEVERITY_NUMBER_WARN2" => Some(Self::Warn2),
            "SEVERITY_NUMBER_WARN3" => Some(Self::Warn3),
            "SEVERITY_NUMBER_WARN4" => Some(Self::Warn4),
            "SEVERITY_NUMBER_ERROR" => Some(Self::Error),
            "SEVERITY_NUMBER_ERROR2" => Some(Self::Error2),
            "SEVERITY_NUMBER_ERROR3" => Some(Self::Error3),
            "SEVERITY_NUMBER_ERROR4" => Some(Self::Error4),
            "SEVERITY_NUMBER_FATAL" => Some(Self::Fatal),
            "SEVERITY_NUMBER_FATAL2" => Some(Self::Fatal2),
            "SEVERITY_NUMBER_FATAL3" => Some(Self::Fatal3),
            "SEVERITY_NUMBER_FATAL4" => Some(Self::Fatal4),
            _ => None,
        }
    }
}
/// LogRecordFlags represents constants used to interpret the
/// LogRecord.flags field, which is protobuf 'fixed32' type and is to
/// be used as bit-fields. Each non-zero value defined in this enum is
/// a bit-mask.  To extract the bit-field, for example, use an
/// expression like:
///
///    (logRecord.flags & LOG_RECORD_FLAGS_TRACE_FLAGS_MASK)
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogRecordFlags {
    /// The zero value for the enum. Should not be used for comparisons.
    /// Instead use bitwise "and" with the appropriate mask as shown above.
    DoNotUse = 0,
    /// Bits 0-7 are used for trace flags.
    TraceFlagsMask = 255,
}
impl LogRecordFlags {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LogRecordFlags::DoNotUse => "LOG_RECORD_FLAGS_DO_NOT_USE",
            LogRecordFlags::TraceFlagsMask => "LOG_RECORD_FLAGS_TRACE_FLAGS_MASK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOG_RECORD_FLAGS_DO_NOT_USE" => Some(Self::DoNotUse),
            "LOG_RECORD_FLAGS_TRACE_FLAGS_MASK" => Some(Self::TraceFlagsMask),
            _ => None,
        }
    }
}
