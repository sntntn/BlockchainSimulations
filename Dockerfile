FROM rust:1.89 AS rust-builder
WORKDIR /app

COPY rustCode ./rustCode
WORKDIR /app/rustCode
RUN cargo build --release
# ----------------------------
FROM golang:1.25 AS go-builder
WORKDIR /app

COPY goCode ./goCode

COPY --from=rust-builder \
    /app/rustCode/target/release/librpc.so \
    /app/goCode/libs/librpc.so

WORKDIR /app/goCode/goApp
ENV CGO_ENABLED=1
RUN go build -o app


# ----------------------------
FROM ubuntu:24.04
WORKDIR /app/goCode/goApp

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libc6 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=go-builder \
    /app/goCode/goApp/app \
    ./app

COPY --from=go-builder \
    /app/goCode/libs/librpc.so \
    ../libs/librpc.so

COPY goCode/.env ../.env

ENV LD_LIBRARY_PATH=/app/goCode/libs

CMD ["./app"]