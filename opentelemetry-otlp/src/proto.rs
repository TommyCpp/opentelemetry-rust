#[cfg(feature = "tonic")]
pub mod collector {
    pub mod metrics {
        pub mod v1 {
            tonic::include_proto!("opentelemetry.proto.collector.metrics.v1");
        }
    }

    pub mod trace {
        pub mod v1 {
            tonic::include_proto!("opentelemetry.proto.collector.trace.v1");
        }
    }
}

#[cfg(feature = "tonic")]
pub mod common {
    pub mod v1 {
        tonic::include_proto!("opentelemetry.proto.common.v1");
    }
}

#[cfg(feature = "tonic")]
pub mod metrics {
    pub mod v1 {
        tonic::include_proto!("opentelemetry.proto.metrics.v1");
    }
}

#[cfg(feature = "tonic")]
pub mod resource {
    pub mod v1 {
        tonic::include_proto!("opentelemetry.proto.resource.v1");
    }
}

#[cfg(feature = "tonic")]
pub mod trace {
    pub mod v1 {
        tonic::include_proto!("opentelemetry.proto.trace.v1");
    }
}

#[cfg(feature = "grpc-sys")]
pub mod grpcio {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));

    pub(crate) mod trace_service_grpc;
    pub(crate) mod metrics_service_grpc;
}
