# MCP Inspector

You can use MCP Inspector to explore and test the prometheus-mcp server.

## Add server (Docker)

When adding a server, set:

- command: `docker`
- args:
  - `run`, `--rm`, `-i`,
  - `brenoepics/prometheus-mcp:latest`,
  - `--mcp`,
  - `--prometheus-url`, `http://host.docker.internal:9090`

## Linux notes

- If `host.docker.internal` does not resolve, either use `http://127.0.0.1:9090` or add `--add-host=host.docker.internal:host-gateway` to the `docker run` args.

## Troubleshooting

- Ensure Prometheus is reachable at the configured URL.
- See [Installation](/installation) and [Docker](/docker) for base setup guidance.

