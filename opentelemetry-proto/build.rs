// Grpc related files used by tonic are generated here. Those files re-generate for each build
// so it's up to date.
//
// Grpc related files used by grpcio are maintained at src/proto/grpcio. tests/grpc_build.rs makes
// sure they are up to date.
use std::path::PathBuf;

fn main() {
    #[cfg(not(feature = "build-server"))]
    let build_server = false;
    #[cfg(feature = "build-server")]
    let build_server = true;

    #[cfg(feature = "gen-tonic")]
    {
        let out_dir = PathBuf::from(
            std::env::var("OUT_DIR").expect("OUT_DIR should be set by cargo but can't find"),
        )
        .join("tonic");
        std::fs::create_dir_all(out_dir.clone()).expect("cannot create output dir");
        tonic_build::configure()
                .build_server(build_server)
                .build_client(true)
                .format(false)
                .out_dir(out_dir)
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
    }

    #[cfg(feature = "gen-prost")]
    {
        let out_dir = PathBuf::from(
            std::env::var("OUT_DIR").expect("OUT_DIR should be set by cargo but can't find"),
        )
        .join("prost");
        std::fs::create_dir_all(out_dir.clone()).expect("cannot create output dir");
        prost_build::Config::new()
                .out_dir(out_dir)
                .compile_protos(
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
    }
}
