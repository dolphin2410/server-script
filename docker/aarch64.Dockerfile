FROM rust:latest AS builder

WORKDIR /app 

COPY . .
 
RUN apt update && apt upgrade -y 
RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross openssl libssl-dev
 
RUN rustup target add aarch64-unknown-linux-gnu 
RUN rustup toolchain install stable-aarch64-unknown-linux-gnu

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc 
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc 
ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

RUN cargo update
RUN cargo build --target aarch64-unknown-linux-gnu --release
 
FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/server-script /tmp/aarch64

CMD ["cp", "/tmp/aarch64", "/app/server-script"]