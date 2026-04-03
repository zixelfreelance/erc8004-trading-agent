# ---------- Stage 1: Build ----------
FROM rust:1.82-bookworm AS builder

WORKDIR /build

# Clone adk-rust (path dependency expects ../adk-rust/adk-rust)
RUN git clone --depth 1 https://github.com/zixelfreelance/adk-rust.git /build/adk-rust

# Copy trading-agent source
COPY . /build/trading-agent

WORKDIR /build/trading-agent

# Build release binary
RUN cargo build --release

# ---------- Stage 2: Runtime ----------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/trading-agent/target/release/trading-agent /usr/local/bin/trading-agent

ENV AGENT_HTTP_PORT=10000
EXPOSE 10000

CMD ["trading-agent"]
