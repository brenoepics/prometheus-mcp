use clap::{Parser, Subcommand};
use prometheus_mcp::mcp::compat;
use prometheus_mcp::mcp::exporter;
use prometheus_mcp::mcp::metrics;
use prometheus_mcp::mcp::tools::{
    prometheus_get_label_values, prometheus_get_metadata, prometheus_get_series,
    prometheus_list_metrics, prometheus_query, prometheus_query_range, register_tools,
    PrometheusGetLabelValuesRequest, PrometheusGetMetadataRequest, PrometheusGetSeriesRequest,
    PrometheusListMetricsRequest, PrometheusQueryRangeRequest, PrometheusQueryRequest,
};
use prometheus_mcp::mcp::types::{
    CancelledNotification, JsonRpcError, JsonRpcResponse, ToolCallRequestParams,
};
use prometheus_mcp::mcp::utilities::*;
use rpc_router::{Error, Handler, Request, Router, RouterBuilder};
use serde_json::{json, Value};
use signal_hook::consts::SIGTERM;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::thread;
use prometheus_mcp::mcp::prometheus_config::PrometheusConfig;
use prometheus_mcp::mcp::repository::{set_repository, HttpPrometheusRepository};
use std::sync::Arc;

/// Build the JSON-RPC router with prompts, resources, and tool handlers.
fn build_rpc_router() -> Router {
    let builder = RouterBuilder::default()
        .append_dyn("initialize", initialize.into_dyn())
        .append_dyn("ping", ping.into_dyn())
        .append_dyn("resources/list", compat::compat_resources_list.into_dyn())
        .append_dyn(
            "resources/templates/list",
            compat::compat_resource_templates_list.into_dyn(),
        )
        .append_dyn("prompts/list", compat::compat_prompts_list.into_dyn());
    let builder = register_tools(builder);
    builder.build()
}

#[derive(Subcommand, Debug)]
/// CLI subcommands for interacting with Prometheus directly.
enum PromCmd {
    /// Instant query
    Query {
        #[arg(long)]
        query: String,
        #[arg(long)]
        time: Option<String>,
    },
    /// Range query
    Range {
        #[arg(long)]
        query: String,
        #[arg(long)]
        start: String,
        #[arg(long)]
        end: String,
        #[arg(long)]
        step: String,
    },
    /// List metric names
    ListMetrics,
    /// Get metric metadata
    Metadata {
        #[arg(long)]
        metric: String,
    },
    /// Get a series for selectors (repeat --selector)
    Series {
        #[arg(long = "selector")]
        selectors: Vec<String>,
    },
    /// Get label values
    LabelValues {
        #[arg(long = "label")]
        label_name: String,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// start MCP server (stdio JSON-RPC)
    #[arg(long, default_value = "false")]
    mcp: bool,
    /// Prometheus server URL
    #[arg(long, env = "PROMETHEUS_URL")]
    prometheus_url: Option<String>,
    /// Basic auth username (or set PROMETHEUS_USERNAME)
    #[arg(long, env = "PROMETHEUS_USERNAME")]
    prometheus_username: Option<String>,
    /// Basic auth password (or set PROMETHEUS_PASSWORD)
    #[arg(long, env = "PROMETHEUS_PASSWORD")]
    prometheus_password: Option<String>,
    /// Enable Prometheus metrics exporter (HTTP /metrics)
    #[arg(long, default_value = "false")]
    metrics_exporter: bool,
    /// Port to expose Prometheus metrics on (when --metrics-exporter is enabled)
    #[arg(long, default_value = "9091")]
    metrics_port: u16,
    /// Prometheus commands (CLI mode)
    #[command(subcommand)]
    cmd: Option<PromCmd>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Start from environment configuration, then apply CLI overrides if provided
    let mut cfg = PrometheusConfig::from_env();
    if let Some(url) = args.prometheus_url.clone() {
        cfg.url = url;
    }
    if let Some(user) = args.prometheus_username.clone() {
        cfg.username = Some(user);
    }
    if let Some(pass) = args.prometheus_password.clone() {
        cfg.password = Some(pass);
    }

    match HttpPrometheusRepository::new(cfg) {
        Ok(repo) => set_repository(Arc::new(repo)),
        Err(e) => {
            eprintln!("Failed to initialize Prometheus repository: {}", e);
            return;
        }
    }

    if let Some(cmd) = &args.cmd {
        // CLI mode: run a single Prometheus command and exit
        run_cli_command(cmd).await;
        return;
    }

    if !args.mcp {
        eprintln!("No command provided. Use --help for usage or pass --mcp to start the server.");
        return;
    }

    metrics::init_metrics();

    // Start exporter only if explicitly enabled
    let (metrics_handle, _metrics_shutdown) = if args.metrics_exporter {
        let (handle, shutdown) = exporter::create_metrics_server(args.metrics_port);
        println!(
            "Metrics server listening on http://0.0.0.0:{}/metrics",
            args.metrics_port
        );
        (Some(handle), Some(shutdown))
    } else {
        (None, None)
    };

    // Graceful shutdown on SIGINT/SIGTERM
    let mut signals = match Signals::new([SIGTERM, SIGINT]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to register signal handler: {}", e);
            return;
        }
    };
    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        if let Some(_sig) = signals.forever().next() {
            graceful_shutdown();
            let _ = tx.send(());
            std::process::exit(0);
        }
    });

    thread::spawn(move || {
        let _ = rx.recv();
    });

    // Process JSON-RPC from MCP client
    let router = build_rpc_router();
    let mut line = String::new();
    let input = io::stdin();
    let mut logging_file = match OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/mcp.jsonl")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open log file: {}", e);
            return;
        }
    };

    metrics::increment_active_connections();

    loop {
        match input.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to read stdin: {}", e);
                break;
            }
        }
        let line = std::mem::take(&mut line);
        let _ = writeln!(logging_file, "{}", line);
        if line.is_empty() {
            continue;
        }

        if let Ok(json_value) = serde_json::from_str::<Value>(&line) {
            if json_value.is_object() && json_value.get("id").is_none() {
                if let Some(method) = json_value.get("method") {
                    if method == "notifications/initialized" {
                        notifications_initialized();
                    } else if method == "notifications/cancelled" {
                        if let Some(params_value) = json_value.get("params") {
                            if let Ok(cancel_params) = serde_json::from_value::<CancelledNotification>(
                                params_value.clone(),
                            ) {
                                notifications_cancelled(cancel_params);
                            }
                        }
                    }
                }
                continue;
            }
            if let Ok(mut rpc_request) = Request::from_value(json_value) {
                let id = rpc_request.id.clone();
                if rpc_request.method == "tools/call" {
                    if let Some(raw_params) = rpc_request.params.take() {
                        if let Ok(params) =
                            serde_json::from_value::<ToolCallRequestParams>(raw_params)
                        {
                            if !params.name.is_empty() {
                                metrics::record_tool_call(&params.name);
                            }
                            rpc_request = Request {
                                id: id.clone(),
                                method: params.name,
                                params: params.arguments,
                            };
                        }
                    }
                }
                match router.call(rpc_request).await {
                    Ok(call_response) => {
                        if !call_response.value.is_null() {
                            let response = JsonRpcResponse::new(id, call_response.value.clone());
                            if let Ok(response_json) = serde_json::to_string(&response) {
                                let _ = writeln!(logging_file, "{}\n", &response_json);
                                println!("{}", response_json);
                            }
                        }
                    }
                    Err(error) => match &error.error {
                        Error::Handler(handler) => {
                            if let Some(error_value) = handler.get::<Value>() {
                                let json_error =
                                    json!({ "jsonrpc": "2.0", "error": error_value, "id": id });
                                if let Ok(response) = serde_json::to_string(&json_error) {
                                    let _ = writeln!(logging_file, "{}\n", &response);
                                    println!("{}", response);
                                }
                            }
                        }
                        _ => {
                            let json_error = JsonRpcError::new(id, -1, "Invalid json-rpc call");
                            if let Ok(response) = serde_json::to_string(&json_error) {
                                let _ = writeln!(logging_file, "{}\n", &response);
                                println!("{}", response);
                            }
                        }
                    },
                }
            }
        }
    }

    metrics::decrement_active_connections();

    // Join exporter if it was started
    if let Some(handle) = metrics_handle {
        let _ = handle.await;
    }
}

/// Execute a single CLI command using the same tool handlers as the MCP server.
async fn run_cli_command(cmd: &PromCmd) {
    match cmd {
        PromCmd::Query { query, time } => {
            let res = prometheus_query(PrometheusQueryRequest {
                query: query.clone(),
                time: time.clone(),
            })
            .await;
            print_tool_result(res);
        }
        PromCmd::Range {
            query,
            start,
            end,
            step,
        } => {
            let res = prometheus_query_range(PrometheusQueryRangeRequest {
                query: query.clone(),
                start: start.clone(),
                end: end.clone(),
                step: step.clone(),
            })
            .await;
            print_tool_result(res);
        }
        PromCmd::ListMetrics => {
            let res = prometheus_list_metrics(PrometheusListMetricsRequest {}).await;
            print_tool_result(res);
        }
        PromCmd::Metadata { metric } => {
            let res = prometheus_get_metadata(PrometheusGetMetadataRequest {
                metric: metric.clone(),
            })
            .await;
            print_tool_result(res);
        }
        PromCmd::Series { selectors } => {
            let res = prometheus_get_series(PrometheusGetSeriesRequest {
                match_strings: selectors.clone(),
            })
            .await;
            print_tool_result(res);
        }
        PromCmd::LabelValues { label_name } => {
            let res = prometheus_get_label_values(PrometheusGetLabelValuesRequest {
                label_name: label_name.clone(),
            })
            .await;
            print_tool_result(res);
        }
    }
}

/// Pretty-print tool handler results or errors for CLI usage.
fn print_tool_result(res: rpc_router::HandlerResult<prometheus_mcp::mcp::types::CallToolResult>) {
    match res {
        Ok(r) => {
            if let Ok(s) = serde_json::to_string_pretty(&r) {
                println!("{}", s);
            }
        }
        Err(e) => {
            eprintln!("error: {:?}", e);
        }
    }
}
