# MCP Tools

The MCP server exposes the following tools. Input schemas match the CLI subcommands.

::: tip
All results are returned as text content containing pretty-printed JSON from the Prometheus HTTP API.
:::

## prometheus_query

- Description: Execute a Prometheus instant query

Parameters

| Name    | Type   | Required | Notes                                                |
|---------|--------|----------|------------------------------------------------------|
| `query` | string | yes      | PromQL query                                         |
| `time`  | string | no       | Evaluation timestamp (RFC3339 or Unix epoch seconds) |

::: details Example call

```json
{
  "method": "tools/call",
  "params": {
    "name": "prometheus_query",
    "arguments": {
      "query": "up",
      "time": "2025-09-27T12:00:00Z"
    }
  }
}
```

:::

## prometheus_query_range

- Description: Execute a Prometheus range query

Parameters

| Name    | Type   | Required | Notes                   |
|---------|--------|----------|-------------------------|
| `query` | string | yes      | PromQL query            |
| `start` | string | yes      | Range start time        |
| `end`   | string | yes      | Range end time          |
| `step`  | string | yes      | e.g., `30s`, `1m`, `1h` |

## prometheus_list_metrics

- Description: List all metric names (values of the `__name__` label)

Parameters

| Name | Type | Required | Notes                       |
|------|------|----------|-----------------------------|
| —    | —    | —        | This tool has no parameters |

## prometheus_get_metadata

- Description: Get metadata about a specific metric

Parameters

| Name     | Type   | Required | Notes       |
|----------|--------|----------|-------------|
| `metric` | string | yes      | Metric name |

## prometheus_get_series

- Description: Get time series data for selectors

Parameters

| Name            | Type                | Required | Notes                        |
|-----------------|---------------------|----------|------------------------------|
| `match_strings` | array&lt;string&gt; | yes      | One or more series selectors |

::: details Example call

```json
{
  "name": "prometheus_get_series",
  "arguments": {
    "match_strings": [
      "up",
      "node_cpu_seconds_total{mode=\"idle\"}"
    ]
  }
}
```

:::

## prometheus_get_label_values

- Description: Get all label values for a specific label

Parameters

| Name         | Type   | Required | Notes                        |
|--------------|--------|----------|------------------------------|
| `label_name` | string | yes      | Label key to list values for |
