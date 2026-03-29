FROM rust:latest AS builder

WORKDIR /app 

COPY . .
 
RUN apt update && apt upgrade -y 
RUN apt install -y g++-mingw-w64-x86-64 openssl libssl-dev
 
RUN rustup target add x86_64-pc-windows-gnu
RUN cargo update
RUN cargo build --target x86_64-pc-windows-gnu --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/x86_64-pc-windows-gnu/release/server-script.exe /tmp/win64.exe

CMD ["cp", "/tmp/win64.exe", "/app/server-script.exe"]