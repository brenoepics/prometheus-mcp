use std::env;
use std::time::Duration;

/// Configuration for Prometheus
#[derive(Debug, Clone)]
pub struct PrometheusConfig {
    /// URL of the Prometheus server
    pub url: String,
    /// Timeout for Prometheus API requests in seconds
    pub timeout: Duration,
    /// Number of retries for Prometheus API requests
    pub retries: u32,
    /// Backoff between retries in milliseconds
    pub retry_backoff_ms: u64,
    /// Minimal interval between requests in milliseconds (simple rate limit)
    pub min_request_interval_ms: Option<u64>,
    /// Cache TTL for metadata/labels in seconds
    pub cache_ttl_secs: Option<u64>,
    /// Basic auth username
    pub username: Option<String>,
    /// Basic auth password
    pub password: Option<String>,
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:9090".to_string(),
            timeout: Duration::from_secs(10),
            retries: 3,
            retry_backoff_ms: 500,
            min_request_interval_ms: None,
            cache_ttl_secs: None,
            username: None,
            password: None,
        }
    }
}

impl PrometheusConfig {
    /// Create a new PrometheusConfig from environment variables
    pub fn from_env() -> Self {
        let url =
            env::var("PROMETHEUS_URL").unwrap_or_else(|_| "http://localhost:9090".to_string());

        let timeout_secs = env::var("PROMETHEUS_TIMEOUT")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(10);

        let retries = env::var("PROMETHEUS_RETRIES")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(3);

        let retry_backoff_ms = env::var("PROMETHEUS_RETRY_BACKOFF_MS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(500);

        let min_request_interval_ms = env::var("PROMETHEUS_MIN_INTERVAL_MS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok());

        let cache_ttl_secs = env::var("PROMETHEUS_CACHE_TTL_SECS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok());

        let username = env::var("PROMETHEUS_USERNAME").ok();
        let password = env::var("PROMETHEUS_PASSWORD").ok();

        Self {
            url,
            timeout: Duration::from_secs(timeout_secs),
            retries,
            retry_backoff_ms,
            min_request_interval_ms,
            cache_ttl_secs,
            username,
            password,
        }
    }

    /// Create a new PrometheusConfig from a map of values (useful for tests)
    #[cfg(test)]
    pub fn from_map(map: &std::collections::HashMap<&str, &str>) -> Self {
        let url = map
            .get("PROMETHEUS_URL")
            .map(|s| s.to_string())
            .unwrap_or_else(|| "http://localhost:9090".to_string());

        let timeout_secs = map
            .get("PROMETHEUS_TIMEOUT")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(10);

        let retries = map
            .get("PROMETHEUS_RETRIES")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(3);

        let retry_backoff_ms = map
            .get("PROMETHEUS_RETRY_BACKOFF_MS")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(500);

        let min_request_interval_ms = map
            .get("PROMETHEUS_MIN_INTERVAL_MS")
            .and_then(|s| s.parse::<u64>().ok());

        let cache_ttl_secs = map
            .get("PROMETHEUS_CACHE_TTL_SECS")
            .and_then(|s| s.parse::<u64>().ok());

        let username = map.get("PROMETHEUS_USERNAME").map(|s| s.to_string());
        let password = map.get("PROMETHEUS_PASSWORD").map(|s| s.to_string());

        Self {
            url,
            timeout: Duration::from_secs(timeout_secs),
            retries,
            retry_backoff_ms,
            min_request_interval_ms,
            cache_ttl_secs,
            username,
            password,
        }
    }

    /// Create a new PrometheusConfig with the given URL
    #[allow(dead_code)]
    pub fn with_url(url: String) -> Self {
        Self {
            url,
            ..Default::default()
        }
    }

    /// Create a new PrometheusConfig with the given timeout
    #[allow(dead_code)]
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout = Duration::from_secs(timeout_secs);
        self
    }

    /// Create a new PrometheusConfig with the given number of retries
    #[allow(dead_code)]
    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    /// Set backoff between retries
    #[allow(dead_code)]
    pub fn with_retry_backoff_ms(mut self, ms: u64) -> Self {
        self.retry_backoff_ms = ms;
        self
    }

    /// Set min request interval (rate limit)
    #[allow(dead_code)]
    pub fn with_min_interval_ms(mut self, ms: u64) -> Self {
        self.min_request_interval_ms = Some(ms);
        self
    }

    /// Set cache TTL (seconds)
    #[allow(dead_code)]
    pub fn with_cache_ttl_secs(mut self, secs: u64) -> Self {
        self.cache_ttl_secs = Some(secs);
        self
    }

    /// Set basic auth
    #[allow(dead_code)]
    pub fn with_basic_auth(
        mut self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_from_env_defaults_and_overrides() {
        // Use from_map to avoid mutating global process env in tests
        let empty: HashMap<&str, &str> = HashMap::new();
        let cfg = PrometheusConfig::from_map(&empty);
        assert_eq!(cfg.url, "http://localhost:9090");

        let mut vars: HashMap<&str, &str> = HashMap::new();
        vars.insert("PROMETHEUS_URL", "http://example:9090");
        vars.insert("PROMETHEUS_TIMEOUT", "5");
        vars.insert("PROMETHEUS_RETRIES", "2");
        vars.insert("PROMETHEUS_RETRY_BACKOFF_MS", "10");
        vars.insert("PROMETHEUS_MIN_INTERVAL_MS", "20");
        vars.insert("PROMETHEUS_CACHE_TTL_SECS", "30");
        vars.insert("PROMETHEUS_USERNAME", "u");
        vars.insert("PROMETHEUS_PASSWORD", "p");

        let cfg = PrometheusConfig::from_map(&vars);
        assert_eq!(cfg.url, "http://example:9090");
        assert_eq!(cfg.timeout, std::time::Duration::from_secs(5));
        assert_eq!(cfg.retries, 2);
        assert_eq!(cfg.retry_backoff_ms, 10);
        assert_eq!(cfg.min_request_interval_ms, Some(20));
        assert_eq!(cfg.cache_ttl_secs, Some(30));
        assert_eq!(cfg.username.as_deref(), Some("u"));
        assert_eq!(cfg.password.as_deref(), Some("p"));
    }
}
