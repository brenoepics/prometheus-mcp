# Docker

A multi-stage Dockerfile is provided for small runtime images.

## Build

```bash
docker build -t prometheus-mcp:latest .
```

## Run (MCP server)

::: tip
Use host networking on Linux for easiest connectivity to a host Prometheus on :9090.
:::

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

::: code-group
```bash [Env]
docker run --rm -it \
  -e PROMETHEUS_URL=https://prom.example.com \
  -e PROMETHEUS_USERNAME=api \
  -e PROMETHEUS_PASSWORD=secret \
  prometheus-mcp:latest --mcp
```
```bash [Flags]
docker run --rm -it prometheus-mcp:latest --mcp \
  --prometheus-url https://prom.example.com \
  --prometheus-username api \
  --prometheus-password secret
```
:::

::: warning Secrets
Prefer Docker secrets or environment variables managed by your orchestrator. Avoid baking credentials into images or command history.
:::

## Metrics exporter

Expose internal metrics at /metrics by enabling the exporter and mapping a port:

```bash
docker run --rm -it -p 9091:9091 prometheus-mcp:latest --mcp \
  --metrics-exporter --metrics-port 9091 \
  --prometheus-url http://host.docker.internal:9090
```
