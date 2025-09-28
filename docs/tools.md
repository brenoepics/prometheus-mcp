# MCP Tools

The MCP server exposes the following tools. Input schemas match the CLI subcommands.

All results are returned as text content containing pretty-printed JSON from the Prometheus HTTP API.

## prometheus_query
- Description: Execute a Prometheus instant query
- Parameters:
  - query (string, required): PromQL query
  - time (string, optional): Evaluation timestamp (RFC3339 or Unix epoch seconds)

Example call (conceptual):
```json
{
  "method": "tools/call",
  "params": {
    "name": "prometheus_query",
    "arguments": { "query": "up", "time": "2025-09-27T12:00:00Z" }
  }
}
```

## prometheus_query_range
- Description: Execute a Prometheus range query
- Parameters:
  - query (string, required)
  - start (string, required)
  - end (string, required)
  - step (string, required): e.g., 30s, 1m, 1h

## prometheus_list_metrics
- Description: List all metric names (values of the `__name__` label)
- Parameters: none

## prometheus_get_metadata
- Description: Get metadata about a specific metric
- Parameters:
  - metric (string, required)

## prometheus_get_series
- Description: Get time series data for selectors
- Parameters:
  - match_strings (array<string>, required, minItems: 1)

Example:
```json
{"name": "prometheus_get_series", "arguments": {"match_strings": ["up", "node_cpu_seconds_total{mode=\"idle\"}"]}}
```

## prometheus_get_label_values
- Description: Get all label values for a specific label
- Parameters:
  - label_name (string, required)

