Prometheus MCP Server
=====================

[![Crates.io](https://img.shields.io/crates/v/prometheus-mcp.svg?style=for-the-badge)](https://crates.io/crates/prometheus-mcp)
[![Docs.rs](https://img.shields.io/docsrs/prometheus-mcp?style=for-the-badge)](https://docs.rs/prometheus-mcp)
[![Release CI](https://github.com/brenoepics/prometheus-mcp/actions/workflows/release.yml/badge.svg)](https://github.com/brenoepics/prometheus-mcp/actions/workflows/release.yml)
[![GitHub release](https://img.shields.io/github/v/release/brenoepics/prometheus-mcp?style=for-the-badge)](https://github.com/brenoepics/prometheus-mcp/releases)
[![Docker pulls](https://img.shields.io/docker/pulls/brenoepics/prometheus-mcp?style=for-the-badge)](https://hub.docker.com/r/brenoepics/prometheus-mcp)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=for-the-badge)](LICENSE)

A minimal Model Context Protocol (MCP) server focused on reading from Prometheus. It exposes Prometheus discovery and query tools to MCP-compatible apps and includes a convenient CLI for local queries.

Highlights

- Instant and range queries via Prometheus HTTP API
- Discovery helpers: list metrics, get metadata, series selectors, label values
- Optional internal metrics exporter at /metrics (disabled by default)
- Works as a stdio MCP server or a one-off CLI

Container images
----------------

Images are published to both Docker Hub and GHCR:
- Docker Hub: `brenoepics/prometheus-mcp`
- GHCR: `ghcr.io/brenoepics/prometheus-mcp`

Quickstart
----------

Pick your preferred install method.

- From crates.io (installs the `prometheus-mcp` binary):

```bash
cargo install prometheus-mcp
prometheus-mcp --help
```

- Prebuilt binaries (GitHub Releases):
  - Download the latest release for your OS/arch:
    https://github.com/brenoepics/prometheus-mcp/releases

- Docker (pull from Docker Hub or GHCR):

```bash
# Docker Hub
docker pull brenoepics/prometheus-mcp:latest
# or GHCR
docker pull ghcr.io/brenoepics/prometheus-mcp:latest

# Run the MCP server against a local Prometheus (pick one image)
docker run --rm -it brenoepics/prometheus-mcp:latest --mcp \
  --prometheus-url http://host.docker.internal:9090
```

Installation
-----------

Build from source (Rust):

```bash
cargo build --release
# binary at ./target/release/prometheus-mcp
```

Or build a Docker image locally:

```bash
docker build -t prometheus-mcp:latest .
```

Usage (CLI)
-----------

The CLI mirrors the tools exposed over MCP.

- Instant query

```bash
prometheus-mcp query --query 'up' --prometheus-url http://localhost:9090
# optionally set an evaluation time
prometheus-mcp query --query 'up' --time '2025-09-27T12:00:00Z'
```

- Range query

```bash
prometheus-mcp range --query 'rate(http_requests_total[5m])' \
  --start '2025-09-27T12:00:00Z' --end '2025-09-27T13:00:00Z' --step '30s'
```

- List metric names

```bash
prometheus-mcp list-metrics
```

- Metric metadata

```bash
prometheus-mcp metadata --metric 'up'
```

- Series selectors (repeat --selector)

```bash
prometheus-mcp series --selector 'up' --selector 'node_cpu_seconds_total{mode="idle"}'
```

- Label values

```bash
prometheus-mcp label-values --label 'job'
```

MCP server (stdio)
------------------

Start the MCP server over stdio:

```bash
prometheus-mcp --mcp --prometheus-url http://localhost:9090
```

Optional: enable internal metrics at /metrics (default off):

```bash
prometheus-mcp --mcp --metrics-exporter --metrics-port 9091
```

Running in Docker
-----------------

Use the published image from Docker Hub (or GHCR alternative shown):

```bash
# Start the MCP server (macOS/Windows: host.docker.internal works; Linux see alternatives below)
docker run --rm -it brenoepics/prometheus-mcp:latest --mcp \
  --prometheus-url http://host.docker.internal:9090
```

Linux alternatives when Prometheus runs on the host:

```bash
# Use host networking (Linux only)
docker run --rm -it --network host brenoepics/prometheus-mcp:latest --mcp \
  --prometheus-url http://localhost:9090

# Without host network: map host gateway and use host.docker.internal
docker run --rm -it --add-host=host.docker.internal:host-gateway \
  brenoepics/prometheus-mcp:latest --mcp \
  --prometheus-url http://host.docker.internal:9090
```

One-off CLI in the container:

```bash
# Instant query
docker run --rm brenoepics/prometheus-mcp:latest query --query 'up' \
  --prometheus-url http://host.docker.internal:9090

# Range query
docker run --rm brenoepics/prometheus-mcp:latest range --query 'rate(http_requests_total[5m])' \
  --start '2025-09-27T12:00:00Z' --end '2025-09-27T13:00:00Z' --step '30s' \
  --prometheus-url http://host.docker.internal:9090
```

Basic Auth
----------

Pass credentials via environment variables or CLI flags.

- Environment variables:

```bash
export PROMETHEUS_URL=https://prom.example.com
export PROMETHEUS_USERNAME=api
export PROMETHEUS_PASSWORD=secret
prometheus-mcp --mcp
```

- CLI flags:

```bash
prometheus-mcp --mcp \
  --prometheus-url https://prom.example.com \
  --prometheus-username api \
  --prometheus-password secret
```

- Docker with env vars:

```bash
docker run --rm -it \
  -e PROMETHEUS_URL=https://prom.example.com \
  -e PROMETHEUS_USERNAME=api \
  -e PROMETHEUS_PASSWORD=secret \
  brenoepics/prometheus-mcp:latest --mcp
```

Configuration
-------------

All settings can be provided via environment variables; some also via flags.

| Name | Type | Default | CLI flag | Description |
|------|------|---------|----------|-------------|
| PROMETHEUS_URL | string (URL) | http://localhost:9090 | --prometheus-url | Base URL of your Prometheus server |
| PROMETHEUS_TIMEOUT | integer (seconds) | 10 | — | HTTP request timeout |
| PROMETHEUS_RETRIES | integer | 3 | — | Number of retries for Prometheus API calls |
| PROMETHEUS_RETRY_BACKOFF_MS | integer (ms) | 500 | — | Time to wait between retries |
| PROMETHEUS_MIN_INTERVAL_MS | integer (ms) | — | — | Minimum interval between query requests (basic rate limit) |
| PROMETHEUS_CACHE_TTL_SECS | integer (seconds) | — | — | TTL for simple in-process caches (list metrics and label values) |
| PROMETHEUS_USERNAME | string | — | --prometheus-username | Basic auth username |
| PROMETHEUS_PASSWORD | string | — | --prometheus-password | Basic auth password |
| — | boolean | false | --mcp | Start MCP server over stdio |
| — | boolean | false | --metrics-exporter | Enable internal Prometheus metrics at /metrics |
| — | integer (port) | 9091 | --metrics-port | Port for /metrics when exporter is enabled |

See docs/configuration.md for notes and examples.

Accessing from Claude Desktop
-----------------------------

Follow the official guide to locate `claude_desktop_config.json`:
https://modelcontextprotocol.io/quickstart/user#for-claude-desktop-users

Minimal Docker-based entry:

```json
{
  "mcpServers": {
    "prometheus": {
      "command": "docker",
      "args": ["run", "--rm", "-i", "brenoepics/prometheus-mcp:latest"]
    }
  }
}
```

With host Prometheus and exporter (macOS/Windows):

```json
{
  "mcpServers": {
    "prometheus": {
      "command": "docker",
      "args": [
        "run", "--rm", "-i",
        "-p", "9091:9091",
        "brenoepics/prometheus-mcp:latest",
        "--mcp",
        "--prometheus-url", "http://host.docker.internal:9090",
        "--metrics-exporter",
        "--metrics-port", "9091"
      ]
    }
  }
}
```

With Basic Auth via environment variables:

```json
{
  "mcpServers": {
    "prometheus": {
      "command": "docker",
      "args": [
        "run", "--rm", "-i",
        "-e", "PROMETHEUS_URL=https://prom.example.com",
        "-e", "PROMETHEUS_USERNAME=api",
        "-e", "PROMETHEUS_PASSWORD=secret",
        "brenoepics/prometheus-mcp:latest", "--mcp"
      ]
    }
  }
}
```

More examples: see docs/claude-desktop.md.

Debugging
---------

Use the MCP Inspector to exercise the server interactively:

```bash
npx @modelcontextprotocol/inspector
```

Connect with transport "STDIO", command `prometheus-mcp`, and optional args `--mcp --prometheus-url http://localhost:9090`.

Logs are appended to /tmp/mcp.jsonl; tail it with:

```bash
tail -f /tmp/mcp.jsonl
```

Security Considerations
-----------------------

- The server does not provide authentication itself; when running outside stdio, keep it on localhost.
- Handle credentials via environment variables or your secret manager. Avoid committing secrets.

License
-------
Apache-2.0
