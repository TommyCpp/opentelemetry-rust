//! # Datadog Exporter Example
//!
//! Sends spans to a datadog-agent collector.
//!
//! ## Usage
//!
//! First run version 7.22.0 or above of the datadog-agent locally as described [here](https://docs.datadoghq.com/agent/)
//!
//! Then run the example to report spans:
//!
//! ```shell
//! $ cargo run
//! ```
//!
//! Traces should appear in the datadog APM dashboard
use opentelemetry::global;
use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::{
    trace::{Span, TraceContextExt, Tracer},
    Key,
};
use opentelemetry_datadog::{new_pipeline, ApiVersion};
use std::thread;
use std::time::Duration;

fn bar() {
    let tracer = global::tracer("component-bar");
    let mut span = tracer.start("bar");
    span.set_attribute(Key::new("span.type").string("sql"));
    span.set_attribute(Key::new("sql.query").string("SELECT * FROM table"));
    thread::sleep(Duration::from_millis(6));
    span.end()
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let tracer = new_pipeline()
        .with_service_name("trace-demo")
        .with_api_version(ApiVersion::Version05)
        .install_simple()?;

    tracer.in_span("foo", |cx| {
        let span = cx.span();
        span.set_attribute(Key::new("span.type").string("web"));
        span.set_attribute(Key::new("http.url").string("http://localhost:8080/foo"));
        span.set_attribute(Key::new("http.method").string("GET"));
        span.set_attribute(Key::new("http.status_code").i64(200));

        thread::sleep(Duration::from_millis(6));
        bar();
        thread::sleep(Duration::from_millis(6));
    });

    shutdown_tracer_provider();

    Ok(())
}
