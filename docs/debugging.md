# Debugging

## MCP Inspector

You can interactively test the MCP server using the official inspector:

```bash
npx @modelcontextprotocol/inspector
```

Then open the URL it prints (e.g., http://localhost:5173) and connect:
- Transport: STDIO
- Command: `prometheus-mcp`
- Args (optional): `--mcp --prometheus-url http://localhost:9090`

Click Connect, then use the Tools tab to list and call the Prometheus tools.

## Logs

The process appends JSON-RPC request/response lines to `/tmp/mcp.jsonl`.

```bash
tail -f /tmp/mcp.jsonl
```

## Common issues

- Connection refused: verify `--prometheus-url` and network reachability (Docker vs host networking).
- 401/403 from Prometheus: configure Basic Auth via env or flags.
- Time format: prefer RFC3339 (e.g., 2025-09-27T12:00:00Z) or Unix epoch seconds.

