FROM rust:alpine AS builder

RUN apk add musl-dev perl make

WORKDIR /app

COPY Cargo.toml Cargo.lock .
COPY src src
RUN cargo build --release

FROM alpine

WORKDIR /app
COPY --from=builder /app/target/release/tapo-rest .
ENTRYPOINT ["./tapo-rest", "/app/devices.json", "--port=80"]
