# Windsurf

You can use prometheus-mcp with Windsurf by adding a new MCP server that runs our Docker image.

## Add server (Docker)

Configure a server with:

- command: `docker`
- args:
  - `run`, `--rm`, `-i`,
  - `brenoepics/prometheus-mcp:latest`,
  - `--mcp`,
  - `--prometheus-url`, `http://host.docker.internal:9090`

## Linux notes

- If `host.docker.internal` does not resolve, either use your host IP (e.g., `http://127.0.0.1:9090`) or add `--add-host=host.docker.internal:host-gateway` to the `docker run` args.

## Troubleshooting

- Ensure Prometheus is reachable at the configured URL.
- See [Installation](/installation) and [Docker](/docker) for more details.

