# Using with Claude Desktop

It’s recommended to use the Docker image when running the MCP server for Claude Desktop.

::: tip Reference
Follow the official instructions to locate/modify your Claude Desktop configuration:

- https://modelcontextprotocol.io/quickstart/user#for-claude-desktop-users
  :::

Edit your `claude_desktop_config.json` to add an entry like one of the following.

## Option A: Run via Docker (recommended)

No credentials required on Prometheus (local instance):

```json
{
  "mcpServers": {
    "prometheus": {
      "command": "docker",
      "args": [
        "run",
        "--rm",
        "-i",
        "prometheus-mcp:latest"
      ]
    }
  }
}
```

With host Prometheus and metrics exporter on port 9091 (macOS/Windows):

```json
{
  "mcpServers": {
    "prometheus": {
      "command": "docker",
      "args": [
        "run",
        "--rm",
        "-i",
        "-p",
        "9091:9091",
        "prometheus-mcp:latest",
        "--mcp",
        "--prometheus-url",
        "http://host.docker.internal:9090",
        "--metrics-exporter",
        "--metrics-port",
        "9091"
      ]
    }
  }
}
```

With Basic Auth (provide env vars):

```json
{
  "mcpServers": {
    "prometheus": {
      "command": "docker",
      "args": [
        "run",
        "--rm",
        "-i",
        "-e",
        "PROMETHEUS_URL=https://prom.example.com",
        "-e",
        "PROMETHEUS_USERNAME=api",
        "-e",
        "PROMETHEUS_PASSWORD=secret",
        "prometheus-mcp:latest",
        "--mcp"
      ]
    }
  }
}
```

Linux: to access the host Prometheus without publishing ports, add `--network host` (works only on Linux):

```json
{
  "mcpServers": {
    "prometheus": {
      "command": "docker",
      "args": [
        "run",
        "--rm",
        "-i",
        "--network",
        "host",
        "prometheus-mcp:latest",
        "--mcp",
        "--prometheus-url",
        "http://localhost:9090"
      ]
    }
  }
}
```

## Option B: Run the local binary

If you built or installed the binary locally:

```json
{
  "mcpServers": {
    "prometheus": {
      "command": "prometheus-mcp",
      "args": [
        "--mcp",
        "--prometheus-url",
        "http://localhost:9090"
      ]
    }
  }
}
```

::: warning Secrets
Prefer environment variables (or a host secret manager) over placing credentials directly in
`claude_desktop_config.json`.
:::
