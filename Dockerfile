FROM rust:1.83-bookworm AS builder
WORKDIR /app

COPY Cargo.* ./

RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

COPY src ./src
COPY README.md LICENSE justfile ./
RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 10001 appuser

COPY --from=builder /app/target/release/prometheus-mcp /usr/local/bin/prometheus-mcp

RUN chown appuser:appuser /usr/local/bin/prometheus-mcp
USER appuser

# Metrics exporter (optional) listens here if enabled
EXPOSE 9091

# Default: start MCP server; override CMD for CLI usage
ENTRYPOINT ["/usr/local/bin/prometheus-mcp"]
CMD ["--mcp"]
