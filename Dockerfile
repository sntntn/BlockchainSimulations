# ---------------- RUST BUILD ----------------
FROM rust:1.89 AS rust-builder
WORKDIR /workspace

COPY rustCode ./rustCode
WORKDIR /workspace/rustCode
RUN cargo build --release

#--------------- GO BUILD --------------------
FROM golang:1.25 AS go-builder
WORKDIR /workspace

COPY goCode ./goCode

# Rust shared lib
COPY --from=rust-builder \
    /workspace/rustCode/target/release/librpc.so \
    /workspace/goCode/libs/librpc.so

WORKDIR /workspace/goCode/goApp
ENV CGO_ENABLED=1
RUN go build -o app

# ------------------ RUNTIME -----------
FROM ubuntu:24.04
WORKDIR /workspace

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libc6 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=go-builder \
    /workspace/goCode/goApp/app \
    /workspace/goCode/goApp/app

COPY --from=rust-builder \
    /workspace/rustCode/target/release/rustCode \
    /workspace/rustCode/rust-app

# Shared lib
COPY --from=go-builder \
    /workspace/goCode/libs/librpc.so \
    /workspace/goCode/libs/librpc.so

ENV LD_LIBRARY_PATH=/workspace/goCode/libs
