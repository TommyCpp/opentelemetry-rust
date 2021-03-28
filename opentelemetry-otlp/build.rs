#[cfg(feature = "grpc-sys")]
extern crate protoc_grpcio;

#[cfg(feature = "grpc-sys")]
use protobuf_codegen::Customize;

#[cfg(feature = "grpc-sys")]
use protoc_grpcio::compile_grpc_protos;
#[cfg(feature = "grpc-sys")]
use std::env;

fn main() {
    #[cfg(feature = "tonic")]
        tonic_build::configure()
        .build_server(std::env::var_os("CARGO_FEATURE_INTEGRATION_TESTING").is_some())
        .build_client(true)
        .format(false)
        .compile(
            &[
                "src/proto/opentelemetry-proto/opentelemetry/proto/common/v1/common.proto",
                "src/proto/opentelemetry-proto/opentelemetry/proto/resource/v1/resource.proto",
                "src/proto/opentelemetry-proto/opentelemetry/proto/trace/v1/trace.proto",
                "src/proto/opentelemetry-proto/opentelemetry/proto/trace/v1/trace_config.proto",
                "src/proto/opentelemetry-proto/opentelemetry/proto/collector/trace/v1/trace_service.proto",
                "src/proto/opentelemetry-proto/opentelemetry/proto/metrics/v1/metrics.proto",
                "src/proto/opentelemetry-proto/opentelemetry/proto/collector/metrics/v1/metrics_service.proto",
            ],
            &["src/proto/opentelemetry-proto"],
        )
        .expect("Error generating protobuf");

    #[cfg(feature = "grpc-sys")]
    {
        let out_put = env::var("OUT_DIR").expect("The OUT_DIR environment variable was not set");
        compile_grpc_protos(
                &[
                    "src/proto/opentelemetry-proto/opentelemetry/proto/common/v1/common.proto",
                    "src/proto/opentelemetry-proto/opentelemetry/proto/resource/v1/resource.proto",
                    "src/proto/opentelemetry-proto/opentelemetry/proto/trace/v1/trace.proto",
                    "src/proto/opentelemetry-proto/opentelemetry/proto/trace/v1/trace_config.proto",
                    "src/proto/opentelemetry-proto/opentelemetry/proto/collector/trace/v1/trace_service.proto",
                    "src/proto/opentelemetry-proto/opentelemetry/proto/metrics/v1/metrics.proto",
                    "src/proto/opentelemetry-proto/opentelemetry/proto/collector/metrics/v1/metrics_service.proto",
                ],
                &["src/proto/opentelemetry-proto/"],
                out_put,
                Some(Customize {
                    expose_fields: Some(true),
                    serde_derive: Some(true),
                    gen_mod_rs: Some(true),
                    ..Default::default()
                }),
            )
                .expect("Error generating protobuf");
    }
}
