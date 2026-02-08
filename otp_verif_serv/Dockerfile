# Используем официальный Rust образ для сборки
FROM rust:1.93 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Минимальный образ для запуска
FROM ubuntu:24.04
WORKDIR /app
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/otp_verif_serv .
EXPOSE 8080
CMD ["./otp_verif_serv"]
