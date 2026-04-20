FROM lukemathwalker/cargo-chef:latest-rust-1-alpine AS chef
WORKDIR /app
# NOTE(pencelheimer): alpine needs musl-dev for compiling some rust crates
# perl and make are often required for openssl/rustls bindings
RUN apk add --no-cache perl make musl-dev

FROM chef AS planner
# NOTE(pencelheimer): preparing dependencies
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# NOTE(pencelheimer): building dependencies
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# NOTE(pencelheimer): building the app
COPY . .
RUN cargo build --release

FROM alpine:latest AS runtime
# NOTE(pencelheimer): running the app
WORKDIR /app

RUN apk add --no-cache tini ca-certificates

COPY --from=builder /app/target/release/event-relay /usr/local/bin/event-relay

ENTRYPOINT ["/sbin/tini", "--", "/usr/local/bin/event-relay"]
