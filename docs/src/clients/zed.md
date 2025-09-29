# Zed

You can add the prometheus-mcp server to Zed by configuring an MCP server that runs our Docker image.

## Add server (Docker)

Use the following when adding a new MCP server:

- command: `docker`
- args:
  - `run`, `--rm`, `-i`,
  - `brenoepics/prometheus-mcp:latest`,
  - `--mcp`,
  - `--prometheus-url`, `http://host.docker.internal:9090`

This runs the server in Docker and points it at Prometheus at `http://host.docker.internal:9090`.

## Linux notes

- `host.docker.internal` may not resolve on some Linux hosts. If needed:
  - Use your host IP directly (e.g., `http://127.0.0.1:9090`), or
  - Add `--add-host=host.docker.internal:host-gateway` to `docker run`.

## Troubleshooting

- Confirm Prometheus is reachable at the configured URL.
- See the general [Installation](/installation) and [Docker](/docker) docs for more options.

