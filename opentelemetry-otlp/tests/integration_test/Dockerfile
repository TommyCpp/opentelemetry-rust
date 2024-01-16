# use whole project as build context
# docker build . -f ./opentelemetry-otlp/tests/integration_test/Dockerfile

# Stage 1: Compile the binary
FROM rust:1.65 as builder

# Create a new empty shell project
COPY . /usr/src/opentelemetry-rust

WORKDIR /usr/src/opentelemetry-rust/opentelemetry-otlp/tests/integration_test

# Build the project.
RUN cargo build --release

# Stage 2: Setup the runtime environment
FROM debian:bullseye-slim

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/opentelemetry-rust/target/release/integration_test_traces /usr/local/bin/integration_test_traces

# Set the binary as the entrypoint of the container
ENTRYPOINT ["integration_test_traces"]