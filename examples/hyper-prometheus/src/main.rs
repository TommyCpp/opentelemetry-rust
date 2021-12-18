use futures_util::{Stream, StreamExt, Future};
use hyper::{
    body,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use once_cell::sync::Lazy;
use opentelemetry::{
    global,
    metrics::{self, Meter, ValueRecorder, Counter},
    sdk::{metrics::{selectors, PushController}},
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use tokio::task::JoinHandle;
use std::time::{Duration, SystemTime};
use opentelemetry::metrics::MetricsError;
use opentelemetry::sdk::export::metrics::stdout;

static PUSH_CONTROLLER: Lazy<PushController> = Lazy::new(|| init_meter().unwrap());
static METER: Lazy<Meter> = Lazy::new(|| {
    Lazy::force(&PUSH_CONTROLLER);
    global::meter("abracadabra")
});
static HTTP_RECORDER: Lazy<ValueRecorder<u64>> = Lazy::new(|| {
    METER.u64_value_recorder("hits.counter")
        .with_description("hit counter")
        .init()
});
static HTTP_REQ_HISTOGRAM: Lazy<ValueRecorder<f64>> = Lazy::new(|| {
    METER.f64_value_recorder("value.duration")
        .with_description("request latencies")
        .init()
});

// Skip first immediate tick from tokio, not needed for async_std.
fn delayed_interval(duration: Duration) -> impl Stream<Item=tokio::time::Instant> {
    println!("call to delayed_interval with duration {:?}", duration);
    opentelemetry::util::tokio_interval_stream(duration).skip(1).map(|i| {
        println!("Interval passed");
        i
    })
}

fn spawn<T>(f: T) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
{
    println!("Called spawn");
    tokio::spawn(f)
}

fn init_meter() -> metrics::Result<PushController> {
    opentelemetry_otlp::new_pipeline()
        .metrics(spawn, delayed_interval)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("https://127.0.0.1:4317/")
                .with_protocol(opentelemetry_otlp::Protocol::Grpc),
        )
        .with_aggregator_selector(selectors::simple::Selector::Inexpensive)
        .with_period(Duration::from_secs(10))
        .with_resource(vec![
            KeyValue::new("service.name", "otlp-test"),
            KeyValue::new("service.namespace", "otlp-test"),
            KeyValue::new("service.instance.id", "test"),
            KeyValue::new("service.version", "0.1.0"),
        ])
        .build()
    // Ok(opentelemetry::sdk::export::metrics::stdout(tokio::spawn, delayed_interval)
    //     .with_formatter(|batch| {
    //         serde_json::to_value(batch)
    //             .map(|value| value.to_string())
    //             .map_err(|err| MetricsError::Other(err.to_string()))
    //     })
    //     .init())
}

fn start_metrics() {
    Lazy::force(&METER);
}

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let request_start = SystemTime::now();

    let attributes = &[
        KeyValue::new("method", req.method().to_string()),
        KeyValue::new("path", req.uri().path().to_owned()),
    ];

    let response = match body::to_bytes(req.into_body()).await {
        Ok(bytes) => {
            Response::builder()
                .status(200)
                .body(Body::from(bytes))
                .unwrap()
        }
        Err(e) => Response::builder()
            .status(500)
            .body(Body::from(e.to_string()))
            .unwrap(),
    };

    // let duration = request_start.elapsed().unwrap_or_default();
    // HTTP_REQ_HISTOGRAM.record(duration.as_secs_f64(), attributes);
    HTTP_RECORDER.record(1, attributes);

    Ok(response)
}

#[tokio::main]
async fn main() {
    start_metrics();

    let addr = ([127, 0, 0, 1], 9898).into();
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(serve_req))
    }));

    if let Err(err) = serve_future.await {
        eprintln!("server error: {}", err);
    }
}