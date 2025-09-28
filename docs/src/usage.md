# Usage

This project provides two ways to interact with Prometheus:
- CLI subcommands for one-off queries
- An MCP server (JSON-RPC over stdio) for use with MCP-compatible apps (for example Claude Desktop)

::: tip
If you’re new here, start with the CLI to verify connectivity, then enable MCP mode.
:::

## CLI

The CLI mirrors the Prometheus tools exposed by the MCP server.

- Instant query

```bash
prometheus-mcp query --query 'up' --prometheus-url http://localhost:9090
# optionally set an evaluation time
prometheus-mcp query --query 'up' --time '2025-09-27T12:00:00Z'
```

::: details Range query
```bash
prometheus-mcp range --query 'rate(http_requests_total[5m])' \
  --start '2025-09-27T12:00:00Z' --end '2025-09-27T13:00:00Z' --step '30s'
```
:::

::: details List metrics
```bash
prometheus-mcp list-metrics
```
:::

::: details Metric metadata
```bash
prometheus-mcp metadata --metric 'up'
```
:::

::: details Series selectors (repeat --selector)
```bash
prometheus-mcp series --selector 'up' --selector 'node_cpu_seconds_total{mode="idle"}'
```
:::

::: details Label values
```bash
prometheus-mcp label-values --label 'job'
```
:::

::: note
Time parameters are passed to Prometheus as-is; use RFC3339 (e.g., 2025-09-27T12:00:00Z), Unix epoch seconds, or Prometheus-compatible times.
:::

## MCP server (stdio JSON-RPC)

Start the MCP server over stdio:

```bash
prometheus-mcp --mcp --prometheus-url http://localhost:9090
```

Optional: enable the built-in Prometheus metrics exporter (HTTP /metrics):

```bash
prometheus-mcp --mcp --metrics-exporter --metrics-port 9091
```

The MCP transport is stdio only. Bindings for HTTP/SSE are not provided by this binary.

## Build from source

::: code-group
```bash [debug]
cargo run -- --help
cargo run -- --mcp --prometheus-url http://localhost:9090
```
```bash [release]
cargo build --release
./target/release/prometheus-mcp --mcp --prometheus-url http://localhost:9090
```
:::
