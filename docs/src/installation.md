# Installation

Use these quick-install links or follow the manual steps below.

## One-click MCP install (Cursor / VS Code)

|                                                                                                                                                                                 Cursor                                                                                                                                                                                  |                                                                                                                                                                                                                       VS Code                                                                                                                                                                                                                       |
|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:|:---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:|
| [![Install MCP Server](https://cursor.com/deeplink/mcp-install-light.svg)](https://cursor.com/en/install-mcp?name=prometheus-mcp&config=%7B%22command%22%3A%22docker%22%2C%22args%22%3A%5B%22run%22%2C%22--rm%22%2C%22-i%22%2C%22brenoepics%2Fprometheus-mcp%3Alatest%22%2C%22--mcp%22%2C%22--prometheus-url%22%2C%22http%3A%2F%2Fhost.docker.internal%3A9090%22%5D%7D) | [![Install on VS Code](https://img.shields.io/badge/Install_on-VS_Code-FF9900?style=flat-square&logo=visualstudiocode&logoColor=white)](https://insiders.vscode.dev/redirect/mcp/install?name=prometheus-mcp&config=%7B%22command%22%3A%22docker%22%2C%22args%22%3A%5B%22run%22%2C%22--rm%22%2C%22-i%22%2C%22brenoepics%2Fprometheus-mcp%3Alatest%22%2C%22--mcp%22%2C%22--prometheus-url%22%2C%22http%3A%2F%2Fhost.docker.internal%3A9090%22%5D%7D) |

::: note
These deep links configure the MCP server to run via Docker. On Linux, consider using `--network host` and
`http://localhost:9090` as the Prometheus URL.
:::

## Manual installation

### Prebuilt binaries

- Download the latest release for your OS/arch:
  https://github.com/brenoepics/prometheus-mcp/releases
- Unpack the archive and place the `prometheus-mcp` binary on your PATH.

### Build from source (Rust)

::: code-group

```bash [Linux/macOS]
cargo install prometheus-mcp
# or
cargo build --release && ./target/release/prometheus-mcp --help
```

```powershell [Windows]
cargo install prometheus-mcp
# or
cargo build --release; .\target\release\prometheus-mcp.exe --help
```

:::

### Docker (recommended for MCP)

- macOS/Windows (no host network):

```bash
docker run --rm -it brenoepics/prometheus-mcp:latest --mcp \
  --prometheus-url http://host.docker.internal:9090
```

- Linux (host network available):

```bash
docker run --rm -it --network host brenoepics/prometheus-mcp:latest --mcp \
  --prometheus-url http://localhost:9090
```

## Windows notes

::: tip
Prefer PowerShell when copying commands from docs.
:::

- Prometheus on Windows host with Docker: use `http://host.docker.internal:9090`.
- To expose the metrics exporter from Docker on Windows/macOS:

```powershell
docker run --rm -it -p 9091:9091 brenoepics/prometheus-mcp:latest --mcp \
  --metrics-exporter --metrics-port 9091 \
  --prometheus-url http://host.docker.internal:9090
```

## Example MCP configs

- Claude Desktop (Docker): see `/docs/claude-desktop.md`.
- Local binary:

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

- Docker (cross-platform):

```json
{
  "mcpServers": {
    "prometheus": {
      "command": "docker",
      "args": [
        "run",
        "--rm",
        "-i",
        "brenoepics/prometheus-mcp:latest",
        "--mcp",
        "--prometheus-url",
        "http://host.docker.internal:9090"
      ]
    }
  }
}
```

