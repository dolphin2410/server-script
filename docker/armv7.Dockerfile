FROM rust:latest AS builder

WORKDIR /app 

COPY . .

RUN dpkg --add-architecture armhf
RUN apt update && apt upgrade -y 
RUN apt install -y g++-arm-linux-gnueabihf libc6-dev-armhf-cross libssl-dev:armhf openssl pkg-config
 
RUN rustup target add armv7-unknown-linux-gnueabihf

ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH_armv7_unknown_linux_gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
ENV CC_armv7_unknown_Linux_gnueabihf=arm-linux-gnueabihf-gcc
ENV CXX_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++

RUN cargo update
RUN cargo build --target armv7-unknown-linux-gnueabihf --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/armv7-unknown-linux-gnueabihf/release/server-script /tmp/armv7

CMD ["cp", "/tmp/armv7", "/app/server-script"]