ARG BASE_IMAGE=clux/muslrust:stable

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS chef

RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin risk-control-service

# Now, we need to build our _real_ Docker container, copying in `flo-ping`
FROM alpine:latest
RUN addgroup -g 1000 rust \
        && adduser -D -s /bin/sh -u 1000 -G rust rust \
        && apk --no-cache add ca-certificates
COPY --from=builder \
    /app/target/x86_64-unknown-linux-musl/release/risk-control-service \
    /usr/local/bin/
USER rust
CMD ["/usr/local/bin/risk-control-service"]
