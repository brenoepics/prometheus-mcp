# Docker

A multi-stage Dockerfile is provided for small runtime images.

## Build

```bash
docker build -t prometheus-mcp:latest .
```

## Run (MCP server)

- Linux (Prometheus on host at :9090) — easiest with host networking:
```bash
docker run --rm -it --network host prometheus-mcp:latest --mcp \
  --prometheus-url http://localhost:9090
```

- macOS/Windows (no host network) — use `host.docker.internal`:
```bash
docker run --rm -it prometheus-mcp:latest --mcp \
  --prometheus-url http://host.docker.internal:9090
```

- Linux without host networking — map host gateway:
```bash
docker run --rm -it --add-host=host.docker.internal:host-gateway \
  prometheus-mcp:latest --mcp \
  --prometheus-url http://host.docker.internal:9090
```

## Run (CLI one-offs)

```bash
# Instant query
docker run --rm prometheus-mcp:latest query --query 'up' \
  --prometheus-url http://host.docker.internal:9090

# Range query
docker run --rm prometheus-mcp:latest range --query 'rate(http_requests_total[5m])' \
  --start '2025-09-27T12:00:00Z' --end '2025-09-27T13:00:00Z' --step '30s' \
  --prometheus-url http://host.docker.internal:9090
```

## Basic Auth

Supply credentials via environment variables or CLI flags.

- Using environment variables:
```bash
docker run --rm -it \
  -e PROMETHEUS_URL=https://prom.example.com \
  -e PROMETHEUS_USERNAME=api \
  -e PROMETHEUS_PASSWORD=secret \
  prometheus-mcp:latest --mcp
```

- Using flags:
```bash
docker run --rm -it prometheus-mcp:latest --mcp \
  --prometheus-url https://prom.example.com \
  --prometheus-username api \
  --prometheus-password secret
```

## Metrics exporter

Expose internal metrics at /metrics by enabling the exporter and mapping a port:

```bash
docker run --rm -it -p 9091:9091 prometheus-mcp:latest --mcp \
  --metrics-exporter --metrics-port 9091 \
  --prometheus-url http://host.docker.internal:9090
```

