# Multi-stage Dockerfile for prometheus-mcp

# 1) Builder stage
FROM rust:1-bookworm AS builder
WORKDIR /app

# Cache dependency compilation
COPY Cargo.toml Cargo.lock ./
# Create a dummy src to cache dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Now copy the real source and build
COPY src ./src
COPY README.md LICENSE justfile ./
RUN cargo build --release

# 2) Runtime stage
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies for reqwest(native-tls) and certs
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 10001 appuser

# Copy the binary
COPY --from=builder /app/target/release/prometheus-mcp /usr/local/bin/prometheus-mcp

# Ownership and permissions
RUN chown appuser:appuser /usr/local/bin/prometheus-mcp
USER appuser

# Metrics exporter (optional) listens here if enabled
EXPOSE 9091

# Default: start MCP server; override CMD for CLI usage
ENTRYPOINT ["/usr/local/bin/prometheus-mcp"]
CMD ["--mcp"]

