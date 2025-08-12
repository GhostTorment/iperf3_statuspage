# -------- Stage 1: Build the binary with musl --------
FROM rust:latest AS builder

# Install musl target and build dependencies
RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && apt-get install -y pkg-config libssl-dev musl-tools

WORKDIR /app

COPY . .

# Build with musl target for static linking
RUN cargo build --release --target x86_64-unknown-linux-musl

# -------- Stage 2: Create minimal runtime image --------
FROM alpine:latest

RUN apk add --no-cache \
    curl \
    ca-certificates \
    iperf3 \
    bash

RUN adduser -D -s /bin/bash iperf3

WORKDIR /app

# Copy the musl static binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/iperf3_statuspage /usr/local/bin/iperf3-statuspage

COPY .env /etc/iperf3-statuspage/.env

EXPOSE 8080

USER iperf3

CMD ["/usr/local/bin/iperf3-statuspage"]
