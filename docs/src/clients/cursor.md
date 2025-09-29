# Cursor

Use one of the options below to add the prometheus-mcp server to Cursor.

## 1) One‑click install (recommended)

Open this link in Cursor to add the server automatically:

- Add server: https://cursor.com/en/install-mcp?name=prometheus-mcp&config=%7B%22command%22%3A%22docker%22%2C%22args%22%3A%5B%22run%22%2C%22--rm%22%2C%22-i%22%2C%22brenoepics%2Fprometheus-mcp%3Alatest%22%2C%22--mcp%22%2C%22--prometheus-url%22%2C%22http%3A%2F%2Fhost.docker.internal%3A9090%22%5D%7D

This config runs the server in Docker and points it at Prometheus at `http://host.docker.internal:9090`.

## 2) Manual add

Add a new MCP server in Cursor settings using the following command and args:

- command: `docker`
- args:
  - `run`, `--rm`, `-i`,
  - `brenoepics/prometheus-mcp:latest`,
  - `--mcp`,
  - `--prometheus-url`, `http://host.docker.internal:9090`

## Linux notes

- `host.docker.internal` may not resolve on some Linux setups. If so, either:
  - Use your host IP directly (e.g., `http://127.0.0.1:9090` if Prometheus is on the host), or
  - Add `--add-host=host.docker.internal:host-gateway` to `docker run` and keep the same URL.

## Troubleshooting

- Ensure Prometheus is reachable at the configured URL.
- See [Installation](/installation) and [Docker](/docker) for more details and variations.

