use crate::mcp::prometheus_config::PrometheusConfig;
use reqwest::{Client, Error as ReqwestError, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Prometheus API client
pub struct PrometheusClient {
    pub(crate) config: PrometheusConfig,
    client: Client,
}

/// Prometheus query result
#[derive(Debug, Deserialize, Serialize)]
pub struct PrometheusQueryResult {
    pub status: String,
    pub data: PrometheusData,
}

/// Prometheus data
#[derive(Debug, Deserialize, Serialize)]
pub struct PrometheusData {
    #[serde(rename = "resultType")]
    pub result_type: String,
    pub result: Vec<PrometheusResult>,
}

/// Prometheus result
#[derive(Debug, Deserialize, Serialize)]
pub struct PrometheusResult {
    pub metric: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<(f64, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<(f64, String)>>,
}

/// Metadata about a metric
#[derive(Debug, Deserialize, Serialize)]
pub struct MetricMetadata {
    pub metric: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub help: String,
    pub unit: String,
}

/// Prometheus API error
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum PrometheusError {
    /// Error from reqwest
    ReqwestError(ReqwestError),
    /// Error from Prometheus API
    ApiError(String),
    /// Error parsing response
    ParseError(String),
    /// Error building HTTP client
    BuildClientError(String),
}

impl fmt::Display for PrometheusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrometheusError::ReqwestError(e) => write!(f, "HTTP error: {}", e),
            PrometheusError::ApiError(msg) => write!(f, "Prometheus API error: {}", msg),
            PrometheusError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            PrometheusError::BuildClientError(msg) => write!(f, "Client build error: {}", msg),
        }
    }
}

impl std::error::Error for PrometheusError {}

impl From<ReqwestError> for PrometheusError {
    fn from(error: ReqwestError) -> Self {
        PrometheusError::ReqwestError(error)
    }
}

impl PrometheusClient {
    /// Create a new PrometheusClient with the given configuration
    pub fn new(config: PrometheusConfig) -> Result<Self, PrometheusError> {
        let builder = Client::builder().timeout(config.timeout);
        let client = builder
            .build()
            .map_err(|e| PrometheusError::BuildClientError(e.to_string()))?;

        Ok(Self { config, client })
    }

    /// Apply basic auth if configured
    fn build_get(&self, url: &str) -> RequestBuilder {
        let rb = self.client.get(url);
        match (&self.config.username, &self.config.password) {
            (Some(user), Some(pass)) => rb.basic_auth(user, Some(pass)),
            _ => rb,
        }
    }

    /// Internal helper: optionally rate-limit, send request, and error-check HTTP status.
    async fn send_request_response(
        &self,
        rb: RequestBuilder,
        rate_limit: bool,
    ) -> Result<reqwest::Response, PrometheusError> {
        if rate_limit {
            if let Some(min_interval) = self.config.min_request_interval_ms {
                tokio::time::sleep(Duration::from_millis(min_interval)).await;
            }
        }
        let response = rb.send().await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(PrometheusError::ApiError(format!(
                "Prometheus API error: {} - {}",
                status, text
            )));
        }
        Ok(response)
    }

    /// Execute an instant query
    pub async fn query(
        &self,
        query: &str,
        time: Option<&str>,
    ) -> Result<PrometheusQueryResult, PrometheusError> {
        let url = format!("{}/api/v1/query", self.config.url);
        let mut params: Vec<(&str, &str)> = vec![("query", query)];

        let time_holder;
        if let Some(t) = time {
            time_holder = t.to_string();
            params.push(("time", &time_holder));
        }

        self.execute_with_retry(url, params).await
    }

    /// Execute a range query
    pub async fn query_range(
        &self,
        query: &str,
        start: &str,
        end: &str,
        step: &str,
    ) -> Result<PrometheusQueryResult, PrometheusError> {
        let url = format!("{}/api/v1/query_range", self.config.url);
        let params = vec![
            ("query", query),
            ("start", start),
            ("end", end),
            ("step", step),
        ];

        self.execute_with_retry(url, params).await
    }

    /// Execute a query with retry
    async fn execute_with_retry<'a>(
        &self,
        url: String,
        params: Vec<(&'a str, &'a str)>,
    ) -> Result<PrometheusQueryResult, PrometheusError> {
        let mut last_error = None;

        for _ in 0..self.config.retries {
            match self.execute_query(&url, &params).await {
                Ok(result) => return Ok(result),
                Err(err) => {
                    last_error = Some(err);
                    // Wait a bit before retrying
                    tokio::time::sleep(Duration::from_millis(self.config.retry_backoff_ms)).await;
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| PrometheusError::ApiError("Maximum retries exceeded".to_string())))
    }

    /// Execute a query
    async fn execute_query<'a>(
        &self,
        url: &str,
        params: &[(&'a str, &'a str)],
    ) -> Result<PrometheusQueryResult, PrometheusError> {
        // Use rate limiting for query endpoints (preserve prior behavior)
        let rb = self.build_get(url).query(params);
        let response = self.send_request_response(rb, true).await?;

        let result: PrometheusQueryResult = response.json().await.map_err(|e| {
            PrometheusError::ParseError(format!("Failed to parse Prometheus response: {}", e))
        })?;

        Ok(result)
    }

    /// Convert a timestamp to a Prometheus-compatible time string
    #[allow(dead_code)]
    pub fn timestamp_to_prometheus_time(timestamp: SystemTime) -> String {
        match timestamp.duration_since(UNIX_EPOCH) {
            Ok(since_epoch) => format!("{}.{}", since_epoch.as_secs(), since_epoch.subsec_nanos()),
            Err(_) => "0".to_string(),
        }
    }

    /// Get the current time as a Prometheus-compatible time string
    #[allow(dead_code)]
    pub fn current_time() -> String {
        Self::timestamp_to_prometheus_time(SystemTime::now())
    }

    /// List all metric names that can be queried from Prometheus
    pub async fn list_metrics(&self) -> Result<Vec<String>, PrometheusError> {
        let url = format!("{}/api/v1/label/__name__/values", self.config.url);
        let rb = self.build_get(&url);
        // No rate limiting here previously; preserve behavior
        let response = self.send_request_response(rb, false).await?;

        let value: Value = response.json().await.map_err(|e| {
            PrometheusError::ParseError(format!("Failed to parse Prometheus response: {}", e))
        })?;
        let mut out = Vec::new();
        if let Some(data) = value.get("data") {
            if let Some(arr) = data.as_array() {
                for item in arr {
                    if let Some(s) = item.as_str() {
                        out.push(s.to_string());
                    }
                }
            }
        }
        Ok(out)
    }

    /// Get metadata about a specific metric
    pub async fn get_metadata(&self, metric: &str) -> Result<Vec<MetricMetadata>, PrometheusError> {
        let url = format!("{}/api/v1/metadata", self.config.url);
        let params = vec![("metric", metric)];

        let rb = self.build_get(&url).query(&params);
        // No rate limiting here previously; preserve behavior
        let response = self.send_request_response(rb, false).await?;

        let result: Value = response.json().await.map_err(|e| {
            PrometheusError::ParseError(format!("Failed to parse Prometheus response: {}", e))
        })?;

        let mut metadata = Vec::new();
        if let Some(data) = result.get("data") {
            if let Some(metric_data) = data.get(metric) {
                if let Some(meta_array) = metric_data.as_array() {
                    for meta in meta_array {
                        if let (Some(type_val), Some(help), Some(unit)) = (
                            meta.get("type").and_then(|v| v.as_str()),
                            meta.get("help").and_then(|v| v.as_str()),
                            meta.get("unit").and_then(|v| v.as_str()),
                        ) {
                            metadata.push(MetricMetadata {
                                metric: metric.to_string(),
                                type_name: type_val.to_string(),
                                help: help.to_string(),
                                unit: unit.to_string(),
                            });
                        }
                    }
                }
            }
        }

        Ok(metadata)
    }

    /// Get time series data for a specific metric with optional label matchers
    pub async fn get_series(
        &self,
        match_strings: Vec<&str>,
    ) -> Result<Vec<HashMap<String, String>>, PrometheusError> {
        let url = format!("{}/api/v1/series", self.config.url);

        // Build the match[] parameters
        let mut params = Vec::new();
        for m in match_strings {
            params.push(("match[]", m));
        }

        let rb = self.build_get(&url).query(&params);
        // No rate limiting here previously; preserve behavior
        let response = self.send_request_response(rb, false).await?;

        let result: Value = response.json().await.map_err(|e| {
            PrometheusError::ParseError(format!("Failed to parse Prometheus response: {}", e))
        })?;

        let mut series = Vec::new();
        if let Some(data) = result.get("data") {
            if let Some(data_array) = data.as_array() {
                for item in data_array {
                    if let Some(obj) = item.as_object() {
                        let mut series_item = HashMap::new();
                        for (k, v) in obj {
                            if let Some(value_str) = v.as_str() {
                                series_item.insert(k.clone(), value_str.to_string());
                            }
                        }
                        series.push(series_item);
                    }
                }
            }
        }

        Ok(series)
    }

    /// Get all label values for a specific label name
    pub async fn get_label_values(&self, label_name: &str) -> Result<Vec<String>, PrometheusError> {
        let url = format!("{}/api/v1/label/{}/values", self.config.url, label_name);

        let rb = self.build_get(&url);
        // No rate limiting here previously; preserve behavior
        let response = self.send_request_response(rb, false).await?;

        let result: Value = response.json().await.map_err(|e| {
            PrometheusError::ParseError(format!("Failed to parse Prometheus response: {}", e))
        })?;

        let mut values = Vec::new();
        if let Some(data) = result.get("data") {
            if let Some(data_array) = data.as_array() {
                for item in data_array {
                    if let Some(value_str) = item.as_str() {
                        values.push(value_str.to_string());
                    }
                }
            }
        }

        Ok(values)
    }
}
