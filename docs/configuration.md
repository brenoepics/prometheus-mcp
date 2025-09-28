# Configuration

This server can be configured via environment variables and a few CLI flags. CLI flags take precedence over environment variables for the same setting.

- Source precedence: CLI flag > environment variable > built-in default

## Quick reference

| Name | Type | Default | CLI flag | Description |
|------|------|---------|----------|-------------|
| PROMETHEUS_URL | string (URL) | http://localhost:9090 | --prometheus-url | Base URL of your Prometheus server |
| PROMETHEUS_TIMEOUT | integer (seconds) | 10 | — | HTTP request timeout |
| PROMETHEUS_RETRIES | integer | 3 | — | Number of retries for Prometheus API calls |
| PROMETHEUS_RETRY_BACKOFF_MS | integer (ms) | 500 | — | Time to wait between retries |
| PROMETHEUS_MIN_INTERVAL_MS | integer (ms) | none | — | If set, enforces a minimum interval between query requests (basic rate limit) |
| PROMETHEUS_CACHE_TTL_SECS | integer (seconds) | none | — | TTL for simple in-process caches (list metrics and label values) |
| PROMETHEUS_USERNAME | string | none | --prometheus-username | Basic auth username |
| PROMETHEUS_PASSWORD | string | none | --prometheus-password | Basic auth password |
| — | boolean | false | --mcp | Start MCP server over stdio |
| — | boolean | false | --metrics-exporter | Enable internal Prometheus metrics at /metrics |
| — | integer (port) | 9091 | --metrics-port | Port to expose the internal /metrics endpoint when enabled |

Notes
- Advanced HTTP behavior (timeout, retries, backoff, rate limiting) is environment-only.
- Caches are per-process and reset on restart.

## Basic Authentication

You can configure Basic Auth using either environment variables or CLI flags.

- Using env vars:
```bash
export PROMETHEUS_URL=https://prom.example.com
export PROMETHEUS_USERNAME=api
export PROMETHEUS_PASSWORD=secret
prometheus-mcp --mcp
```

- Using CLI flags:
```bash
prometheus-mcp --mcp \
  --prometheus-url https://prom.example.com \
  --prometheus-username api \
  --prometheus-password secret
```

- With Docker (env):
```bash
docker run --rm -it \
  -e PROMETHEUS_URL=https://prom.example.com \
  -e PROMETHEUS_USERNAME=api \
  -e PROMETHEUS_PASSWORD=secret \
  prometheus-mcp:latest --mcp
```

Security reminder: avoid committing secrets and prefer Docker/host secret stores when available.

## Metrics Exporter

If enabled with `--metrics-exporter`, the binary exposes its own Prometheus metrics at `/metrics` on `--metrics-port` (default 9091).

```bash
prometheus-mcp --mcp --metrics-exporter --metrics-port 9091
curl -s http://localhost:9091/metrics | head
```

