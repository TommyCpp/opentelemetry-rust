use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Server};
use opentelemetry::{
    global,
    sdk::{
        trace::{self, Sampler},
    },
    trace::{Tracer},
    runtime::Tokio,
};
use opentelemetry_zpages::{tracez, TracezQuerier};
use std::{convert::Infallible, net::SocketAddr};
use hyper::http::{Request, Response};
use std::sync::Arc;
use opentelemetry::trace::{Span, StatusCode};

async fn handler(req: Request<Body>, querier: Arc<TracezQuerier>) -> Result<Response<Body>, Infallible> {
    Ok::<_, Infallible>(match req.uri().path() {
        "/tracez/api/aggregations" => {
            match querier.aggregation().await {
                Ok(resp) => Response::new(Body::from(resp.into_json().unwrap())),
                Err(_) => Response::builder().status(500).body(Body::empty()).unwrap(),
            }
        }
        "/running" => {
            let mut spans = global::tracer("zpages-test").start("running-spans");
            spans.set_status(StatusCode::Ok, "".to_string());
            Response::new(Body::empty())
        }
        _ => {
            println!("{}", req.uri().path());
            Response::builder().status(404).body(Body::empty()).unwrap()
        },
    })
}

#[tokio::main]
async fn main() {
    let (processor, querier) = tracez(5, Tokio);
    let provider = trace::TracerProvider::builder()
        .with_span_processor(processor)
        .build();
    global::set_tracer_provider(provider);
    let querier = Arc::new(querier);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let server = Server::bind(&addr).serve(make_service_fn(move |_conn| {
        let inner = Arc::clone(&querier);
        async move {
            Ok::<_, Infallible>(
                service_fn(move |req| handler(req, Arc::clone(&inner)))
            )
        }
    }));

    println!("Listening on {}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
