pub mod compat;
pub mod exporter;
pub mod metrics;
pub mod prometheus_client;
pub mod prometheus_config;
pub mod repository;
pub mod tools;
pub mod types;
pub mod utilities;

const JSONRPC_VERSION: &str = "2.0";
const PROTOCOL_VERSION: &str = "2024-11-05";
const SERVER_NAME: &str = "prometheus-mcp";
const SERVER_VERSION: &str = "0.1.1";
