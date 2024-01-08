 Feature                                                                                          | Tokio Tracing |
|--------------------------------------------------------------------------------------------------|----------|
| [TracerProvider](specification/trace/api.md#tracerprovider-operations)                           |          |
| Create TracerProvider                                                                            |          |
| Get a Tracer                                                                                     |          |
| Get a Tracer with schema_url                                                                     |          |
| Get a Tracer with scope attributes                                                               |          |
| Associate Tracer with InstrumentationScope                                                       |          |
| Safe for concurrent calls                                                                        |          |
| Shutdown (SDK only required)                                                                     |          |
| ForceFlush (SDK only required)                                                                   |          |
| [Trace / Context interaction](specification/trace/api.md#context-interaction)                    |          |
| Get active Span                                                                                  |          |
| Set active Span                                                                                  |          |
| [Tracer](specification/trace/api.md#tracer-operations)                                           | Optional |
| Create a new Span                                                                                |          |
| Documentation defines adding attributes at span creation as preferred                            |          |
| Get active Span                                                                                  |          |
| Mark Span active                                                                                 |          |
| [SpanContext](specification/trace/api.md#spancontext)                                            |          |
| IsValid                                                                                          | ?, can add as event or metadata |
| IsRemote                                                                                         | ?, can add as event or metadata |
| Conforms to the W3C TraceContext spec                                                            | x        |
| [Span](specification/trace/api.md#span)                                                          | Optional |
| Create root span                                                                                 | +        |
| Create with default parent (active span)                                                         | +        |
| Create with parent from Context                                                                  | +        |
| No explicit parent Span/SpanContext allowed                                                      |          |
| SpanProcessor.OnStart receives parent Context                                                    |          |
| UpdateName                                                                                       | -        |
| User-defined start timestamp                                                                     |          |
| End                                                                                              | +, when guard drops |
| End with timestamp                                                                               | x        |
| IsRecording                                                                                      | ?, can add attributes or metadata |
| IsRecording becomes false after End                                                              | x        |
| Set status with StatusCode (Unset, Ok, Error)                                                    | ?, can add as event or metadata |
| Safe for concurrent calls                                                                        | +        |
| events collection size limit                                                                     | -        |
| attribute collection size limit                                                                  | -        |
| links collection size limit                                                                      | -        |
| [Span attributes](specification/trace/api.md#set-attributes)                                     | Optional |
| SetAttribute                                                                                     | +        |
| Set order preserved                                                                              | X        |
| String type                                                                                      | +        |
| Boolean type                                                                                     | +        |
| Double floating-point type                                                                       | +        |
| Signed int64 type                                                                                | +        |
| Array of primitives (homogeneous)                                                                |          |
| `null` values documented as invalid/undefined                                                    |          |
| Unicode support for keys and string values                                                       |          |
| [Span linking](specification/trace/api.md#specifying-links)                                      | Optional |
| Links can be recorded on span creation                                                           |          |
| Links can be recorded after span creation                                                        |          |
| Links order is preserved                                                                         |          |
| [Span events](specification/trace/api.md#add-events)                                             |          |
| AddEvent                                                                                         |          |
| Add order preserved                                                                              |          |
| Safe for concurrent calls                                                                        |          |
| [Span exceptions](specification/trace/api.md#record-exception)                                   |          |
| RecordException                                                                                  |          |
| RecordException with extra parameters                                                            |          |
| [Sampling](specification/trace/sdk.md#sampling)                                                  | Optional |
| Allow samplers to modify tracestate                                                              |          |
| ShouldSample gets full parent Context                                                            |          |
| Sampler: JaegerRemoteSampler                                                                     |          |
| [New Span ID created also for non-recording Spans](specification/trace/sdk.md#sdk-span-creation) |          |
| [IdGenerators](specification/trace/sdk.md#id-generators)                                         |          |
| [SpanLimits](specification/trace/sdk.md#span-limits)                                             | X        |
| [Built-in `SpanProcessor`s implement `ForceFlush` spec](specification/trace/sdk.md#forceflush-1) |          |
| [Attribute Limits](specification/common/README.md#attribute-limits)                              | X        |
| Fetch InstrumentationScope from ReadableSpan                                                     |          |
