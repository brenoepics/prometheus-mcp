# VS Code

Use one of the options below to add the prometheus-mcp server to VS Code.

## 1) One‑click install (recommended)

Open this link in VS Code Insiders (web) to add the server automatically:

- Add server: https://insiders.vscode.dev/redirect/mcp/install?name=prometheus%20mcp%20server&config=%7B%22command%22%3A%22docker%22%2C%22args%22%3A%5B%22run%22%2C%22--rm%22%2C%22-i%22%2C%22brenoepics%2Fprometheus-mcp%3Alatest%22%2C%22--mcp%22%2C%22--prometheus-url%22%2C%22http%3A%2F%2Fhost.docker.internal%3A9090%22%5D%7D

This config runs the server in Docker and points it at Prometheus at `http://host.docker.internal:9090`.

## 2) Manual add via settings.json

If you prefer to add it manually, configure your MCP servers in VS Code settings (JSON):

```jsonc
{
  "mcp.servers": {
    "prometheus mcp server": {
      "command": "docker",
      "args": [
        "run", "--rm", "-i",
        "brenoepics/prometheus-mcp:latest",
        "--mcp",
        "--prometheus-url", "http://host.docker.internal:9090"
      ]
    }
  }
}
```

## Linux notes

- `host.docker.internal` may not resolve on some Linux setups. If so, either:
  - Use your host IP directly (e.g., `http://127.0.0.1:9090` if Prometheus is on the host), or
  - Add `--add-host=host.docker.internal:host-gateway` to `docker run` and keep the same URL.

## Troubleshooting

- Make sure Prometheus is reachable at the URL you configured.
- Check the server logs (VS Code MCP output) for errors.
- See the general [Installation](/installation) and [Docker](/docker) docs for more options.

