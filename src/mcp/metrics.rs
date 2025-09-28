use lazy_static::lazy_static;
use prometheus::{CounterVec, Gauge, HistogramOpts, HistogramVec, Opts, Registry};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    // RPC requests counter by method
    pub static ref RPC_REQUESTS_TOTAL: CounterVec = CounterVec::new(
        Opts::new("mcp_rpc_requests_total", "Total number of RPC requests"),
        &["method"]
    ).expect("failed to create RPC_REQUESTS_TOTAL");

    // RPC request duration histogram by method
    pub static ref RPC_REQUEST_DURATION_SECONDS: HistogramVec = HistogramVec::new(
        HistogramOpts::new(
            "mcp_rpc_request_duration_seconds",
            "RPC request duration in seconds"
        ).buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0]),
        &["method"]
    ).expect("failed to create RPC_REQUEST_DURATION_SECONDS");

    // Active connections gauge
    pub static ref ACTIVE_CONNECTIONS: Gauge = Gauge::new(
        "mcp_active_connections",
        "Number of active connections"
    ).expect("failed to create ACTIVE_CONNECTIONS");

    // Tool calls counter by tool name
    pub static ref TOOL_CALLS_TOTAL: CounterVec = CounterVec::new(
        Opts::new("mcp_tool_calls_total", "Total number of tool calls"),
        &["tool"]
    ).expect("failed to create TOOL_CALLS_TOTAL");
}

/// Initialize all metrics and register them with the Prometheus registry
pub fn init_metrics() {
    // Register metrics with the registry
    let _ = REGISTRY.register(Box::new(RPC_REQUESTS_TOTAL.clone()));
    let _ = REGISTRY.register(Box::new(RPC_REQUEST_DURATION_SECONDS.clone()));
    let _ = REGISTRY.register(Box::new(ACTIVE_CONNECTIONS.clone()));
    let _ = REGISTRY.register(Box::new(TOOL_CALLS_TOTAL.clone()));
}

/// Record an RPC request with its duration
#[allow(dead_code)]
pub fn record_rpc_request(method: &str, duration_seconds: f64) {
    RPC_REQUESTS_TOTAL.with_label_values(&[method]).inc();
    RPC_REQUEST_DURATION_SECONDS
        .with_label_values(&[method])
        .observe(duration_seconds);
}

/// Record a tool call
pub fn record_tool_call(tool: &str) {
    TOOL_CALLS_TOTAL.with_label_values(&[tool]).inc();
}

/// Increment active connections
pub fn increment_active_connections() {
    ACTIVE_CONNECTIONS.inc();
}

/// Decrement active connections
pub fn decrement_active_connections() {
    ACTIVE_CONNECTIONS.dec();
}

/// Get all metrics as a string
pub fn get_metrics_as_string() -> String {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = Vec::new();
    if encoder.encode(&REGISTRY.gather(), &mut buffer).is_ok() {
        String::from_utf8(buffer).unwrap_or_default()
    } else {
        String::new()
    }
}
