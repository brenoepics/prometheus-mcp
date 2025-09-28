use crate::mcp::metrics;
use crate::mcp::repository::get_repository;
use crate::mcp::types::*;
use maplit::hashmap;
use rpc_router::{Handler, HandlerResult, RouterBuilder, RpcParams};
use serde::{Deserialize, Serialize};

/// register all tools to the router
pub fn register_tools(router_builder: RouterBuilder) -> RouterBuilder {
    router_builder
        .append_dyn("tools/list", tools_list.into_dyn())
        .append_dyn("prometheus_query", prometheus_query.into_dyn())
        .append_dyn("prometheus_query_range", prometheus_query_range.into_dyn())
        .append_dyn(
            "prometheus_list_metrics",
            prometheus_list_metrics.into_dyn(),
        )
        .append_dyn(
            "prometheus_get_metadata",
            prometheus_get_metadata.into_dyn(),
        )
        .append_dyn("prometheus_get_series", prometheus_get_series.into_dyn())
        .append_dyn(
            "prometheus_get_label_values",
            prometheus_get_label_values.into_dyn(),
        )
}

pub async fn tools_list(_request: Option<ListToolsRequest>) -> HandlerResult<ListToolsResult> {
    // Build schemas with additionalProperties=false. Also mirror input_schema to `parameters` for compatibility.
    let query_schema = ToolInputSchema {
        type_name: "object".to_string(),
        properties: hashmap! {
            "query".to_string() => ToolInputSchemaProperty {
                type_name: Some("string".to_owned()),
                description: Some("Prometheus query string".to_owned()),
                enum_values: None,
                items: None,
                min_items: None,
            },
            "time".to_string() => ToolInputSchemaProperty {
                type_name: Some("string".to_owned()),
                description: Some("Evaluation timestamp (RFC3339 or Unix timestamp)".to_owned()),
                enum_values: None,
                items: None,
                min_items: None,
            }
        },
        required: vec!["query".to_string()],
        additional_properties: Some(false),
    };

    let range_schema = ToolInputSchema {
        type_name: "object".to_string(),
        properties: hashmap! {
            "query".to_string() => ToolInputSchemaProperty {
                type_name: Some("string".to_owned()),
                description: Some("Prometheus query string".to_owned()),
                enum_values: None,
                items: None,
                min_items: None,
            },
            "start".to_string() => ToolInputSchemaProperty {
                type_name: Some("string".to_owned()),
                description: Some("Start timestamp (RFC3339 or Unix timestamp)".to_owned()),
                enum_values: None,
                items: None,
                min_items: None,
            },
            "end".to_string() => ToolInputSchemaProperty {
                type_name: Some("string".to_owned()),
                description: Some("End timestamp (RFC3339 or Unix timestamp)".to_owned()),
                enum_values: None,
                items: None,
                min_items: None,
            },
            "step".to_string() => ToolInputSchemaProperty {
                type_name: Some("string".to_owned()),
                description: Some("Query resolution step width (e.g. 30s, 1m, 1h)".to_owned()),
                enum_values: None,
                items: None,
                min_items: None,
            }
        },
        required: vec![
            "query".to_string(),
            "start".to_string(),
            "end".to_string(),
            "step".to_string(),
        ],
        additional_properties: Some(false),
    };

    let list_metrics_schema = ToolInputSchema {
        type_name: "object".to_string(),
        properties: hashmap! {},
        required: vec![],
        additional_properties: Some(false),
    };

    let metadata_schema = ToolInputSchema {
        type_name: "object".to_string(),
        properties: hashmap! {
            "metric".to_string() => ToolInputSchemaProperty {
                type_name: Some("string".to_owned()),
                description: Some("Metric name".to_owned()),
                enum_values: None,
                items: None,
                min_items: None,
            }
        },
        required: vec!["metric".to_string()],
        additional_properties: Some(false),
    };

    let series_schema = ToolInputSchema {
        type_name: "object".to_string(),
        properties: hashmap! {
            "match_strings".to_string() => ToolInputSchemaProperty {
                type_name: Some("array".to_owned()),
                description: Some("Array of Prometheus series selectors (e.g. ['up', 'node_cpu_seconds_total{mode=\"idle\"}'])".to_owned()),
                enum_values: None,
                items: Some(Box::new(ToolInputSchemaProperty {
                    type_name: Some("string".to_owned()),
                    enum_values: None,
                    description: None,
                    items: None,
                    min_items: None,
                })),
                min_items: Some(1),
            }
        },
        required: vec!["match_strings".to_string()],
        additional_properties: Some(false),
    };

    let label_values_schema = ToolInputSchema {
        type_name: "object".to_string(),
        properties: hashmap! {
            "label_name".to_string() => ToolInputSchemaProperty {
                type_name: Some("string".to_owned()),
                description: Some("Label name".to_owned()),
                enum_values: None,
                items: None,
                min_items: None,
            }
        },
        required: vec!["label_name".to_string()],
        additional_properties: Some(false),
    };

    let response = ListToolsResult {
        tools: vec![
            Tool {
                name: "prometheus_query".to_string(),
                description: Some("Execute a Prometheus instant query".to_string()),
                parameters: Some(query_schema.clone()),
                input_schema: query_schema,
            },
            Tool {
                name: "prometheus_query_range".to_string(),
                description: Some("Execute a Prometheus range query".to_string()),
                parameters: Some(range_schema.clone()),
                input_schema: range_schema,
            },
            Tool {
                name: "prometheus_list_metrics".to_string(),
                description: Some(
                    "List all metric names that can be queried from Prometheus".to_string(),
                ),
                parameters: Some(list_metrics_schema.clone()),
                input_schema: list_metrics_schema,
            },
            Tool {
                name: "prometheus_get_metadata".to_string(),
                description: Some("Get metadata about a specific metric".to_string()),
                parameters: Some(metadata_schema.clone()),
                input_schema: metadata_schema,
            },
            Tool {
                name: "prometheus_get_series".to_string(),
                description: Some(
                    "Get time series data for a specific metric with optional label matchers"
                        .to_string(),
                ),
                parameters: Some(series_schema.clone()),
                input_schema: series_schema,
            },
            Tool {
                name: "prometheus_get_label_values".to_string(),
                description: Some("Get all label values for a specific label name".to_string()),
                parameters: Some(label_values_schema.clone()),
                input_schema: label_values_schema,
            },
        ],
        next_cursor: None,
    };
    Ok(response)
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct PrometheusQueryRequest {
    pub query: String,
    pub time: Option<String>,
}

pub async fn prometheus_query(request: PrometheusQueryRequest) -> HandlerResult<CallToolResult> {
    metrics::record_tool_call("prometheus_query");

    let repo = get_repository();
    match repo.query(&request.query, request.time.as_deref()).await {
        Ok(result) => {
            let result_json =
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string());
            Ok(CallToolResult {
                content: vec![CallToolResultContent::Text { text: result_json }],
                is_error: false,
            })
        }
        Err(err) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text {
                text: err_string(&err),
            }],
            is_error: true,
        }),
    }
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct PrometheusQueryRangeRequest {
    pub query: String,
    pub start: String,
    pub end: String,
    pub step: String,
}

pub async fn prometheus_query_range(
    request: PrometheusQueryRangeRequest,
) -> HandlerResult<CallToolResult> {
    metrics::record_tool_call("prometheus_query_range");

    let repo = get_repository();
    match repo
        .query_range(&request.query, &request.start, &request.end, &request.step)
        .await
    {
        Ok(result) => {
            let result_json =
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string());
            Ok(CallToolResult {
                content: vec![CallToolResultContent::Text { text: result_json }],
                is_error: false,
            })
        }
        Err(err) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text {
                text: err_string(&err),
            }],
            is_error: true,
        }),
    }
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct PrometheusListMetricsRequest {}

pub async fn prometheus_list_metrics(
    _request: PrometheusListMetricsRequest,
) -> HandlerResult<CallToolResult> {
    metrics::record_tool_call("prometheus_list_metrics");

    let repo = get_repository();
    match repo.list_metrics().await {
        Ok(metrics) => {
            let result_json =
                serde_json::to_string_pretty(&metrics).unwrap_or_else(|_| "[]".to_string());
            Ok(CallToolResult {
                content: vec![CallToolResultContent::Text { text: result_json }],
                is_error: false,
            })
        }
        Err(err) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text {
                text: err_string(&err),
            }],
            is_error: true,
        }),
    }
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct PrometheusGetMetadataRequest {
    pub metric: String,
}

pub async fn prometheus_get_metadata(
    request: PrometheusGetMetadataRequest,
) -> HandlerResult<CallToolResult> {
    metrics::record_tool_call("prometheus_get_metadata");

    let repo = get_repository();
    match repo.get_metadata(&request.metric).await {
        Ok(metadata) => {
            let result_json =
                serde_json::to_string_pretty(&metadata).unwrap_or_else(|_| "[]".to_string());
            Ok(CallToolResult {
                content: vec![CallToolResultContent::Text { text: result_json }],
                is_error: false,
            })
        }
        Err(err) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text {
                text: err_string(&err),
            }],
            is_error: true,
        }),
    }
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct PrometheusGetSeriesRequest {
    pub match_strings: Vec<String>,
}

pub async fn prometheus_get_series(
    request: PrometheusGetSeriesRequest,
) -> HandlerResult<CallToolResult> {
    metrics::record_tool_call("prometheus_get_series");

    let repo = get_repository();

    // Convert Vec<String> to Vec<&str>
    let match_strings: Vec<&str> = request.match_strings.iter().map(|s| s.as_str()).collect();

    match repo.get_series(match_strings).await {
        Ok(series) => {
            let result_json =
                serde_json::to_string_pretty(&series).unwrap_or_else(|_| "[]".to_string());
            Ok(CallToolResult {
                content: vec![CallToolResultContent::Text { text: result_json }],
                is_error: false,
            })
        }
        Err(err) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text {
                text: err_string(&err),
            }],
            is_error: true,
        }),
    }
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct PrometheusGetLabelValuesRequest {
    pub label_name: String,
}

pub async fn prometheus_get_label_values(
    request: PrometheusGetLabelValuesRequest,
) -> HandlerResult<CallToolResult> {
    metrics::record_tool_call("prometheus_get_label_values");

    let repo = get_repository();
    match repo.get_label_values(&request.label_name).await {
        Ok(values) => {
            let result_json =
                serde_json::to_string_pretty(&values).unwrap_or_else(|_| "[]".to_string());
            Ok(CallToolResult {
                content: vec![CallToolResultContent::Text { text: result_json }],
                is_error: false,
            })
        }
        Err(err) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text {
                text: err_string(&err),
            }],
            is_error: true,
        }),
    }
}

fn err_string(err: &dyn std::fmt::Debug) -> String {
    format!("{:?}", err)
}
