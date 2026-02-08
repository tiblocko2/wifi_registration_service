FROM rust:1.93 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM ubuntu:24.04
WORKDIR /app
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    openssl \
 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/wifi_access_service .
CMD ["./wifi_access_service"]