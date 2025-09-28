use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use async_trait::async_trait;

use crate::mcp::prometheus_client::{
    MetricMetadata, PrometheusClient, PrometheusError, PrometheusQueryResult,
};
use crate::mcp::prometheus_config::PrometheusConfig;

use once_cell::sync::Lazy;

#[async_trait]
pub trait PrometheusRepository: Send + Sync {
    async fn query(
        &self,
        query: &str,
        time: Option<&str>,
    ) -> Result<PrometheusQueryResult, PrometheusError>;
    async fn query_range(
        &self,
        query: &str,
        start: &str,
        end: &str,
        step: &str,
    ) -> Result<PrometheusQueryResult, PrometheusError>;
    async fn list_metrics(&self) -> Result<Vec<String>, PrometheusError>;
    async fn get_metadata(&self, metric: &str) -> Result<Vec<MetricMetadata>, PrometheusError>;
    async fn get_series(
        &self,
        match_strings: Vec<&str>,
    ) -> Result<Vec<std::collections::HashMap<String, String>>, PrometheusError>;
    async fn get_label_values(&self, label_name: &str) -> Result<Vec<String>, PrometheusError>;
}

pub struct HttpPrometheusRepository {
    client: PrometheusClient,
    // Simple caches
    metrics_cache: RwLock<Option<(Instant, Vec<String>)>>, // cache for list_metrics
    labels_cache: RwLock<std::collections::HashMap<String, (Instant, Vec<String>)>>, // per-label cache
    cache_ttl: Duration,
}

impl HttpPrometheusRepository {
    pub fn new(config: PrometheusConfig) -> Result<Self, PrometheusError> {
        let client = PrometheusClient::new(config.clone())?;
        let ttl = config
            .cache_ttl_secs
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(0));
        Ok(Self {
            client,
            metrics_cache: RwLock::new(None),
            labels_cache: RwLock::new(std::collections::HashMap::new()),
            cache_ttl: ttl,
        })
    }

    pub fn from_env() -> Result<Self, PrometheusError> {
        Self::new(PrometheusConfig::from_env())
    }

    fn is_expired(ts: Instant, ttl: Duration) -> bool {
        ttl > Duration::from_secs(0) && ts.elapsed() > ttl
    }
}

#[async_trait]
impl PrometheusRepository for HttpPrometheusRepository {
    async fn query(
        &self,
        query: &str,
        time: Option<&str>,
    ) -> Result<PrometheusQueryResult, PrometheusError> {
        self.client.query(query, time).await
    }

    async fn query_range(
        &self,
        query: &str,
        start: &str,
        end: &str,
        step: &str,
    ) -> Result<PrometheusQueryResult, PrometheusError> {
        self.client.query_range(query, start, end, step).await
    }

    async fn list_metrics(&self) -> Result<Vec<String>, PrometheusError> {
        // Try cache
        if self.cache_ttl > Duration::from_secs(0) {
            if let Some((ts, cached)) = self.metrics_cache.read().unwrap().as_ref() {
                if !Self::is_expired(*ts, self.cache_ttl) {
                    return Ok(cached.clone());
                }
            }
        }
        let fresh = self.client.list_metrics().await?;
        if self.cache_ttl > Duration::from_secs(0) {
            *self.metrics_cache.write().unwrap() = Some((Instant::now(), fresh.clone()));
        }
        Ok(fresh)
    }

    async fn get_metadata(&self, metric: &str) -> Result<Vec<MetricMetadata>, PrometheusError> {
        self.client.get_metadata(metric).await
    }

    async fn get_series(
        &self,
        match_strings: Vec<&str>,
    ) -> Result<Vec<std::collections::HashMap<String, String>>, PrometheusError> {
        self.client.get_series(match_strings).await
    }

    async fn get_label_values(&self, label_name: &str) -> Result<Vec<String>, PrometheusError> {
        if self.cache_ttl > Duration::from_secs(0) {
            if let Some((ts, cached)) = self.labels_cache.read().unwrap().get(label_name) {
                if !Self::is_expired(*ts, self.cache_ttl) {
                    return Ok(cached.clone());
                }
            }
        }
        let fresh = self.client.get_label_values(label_name).await?;
        if self.cache_ttl > Duration::from_secs(0) {
            self.labels_cache
                .write()
                .unwrap()
                .insert(label_name.to_string(), (Instant::now(), fresh.clone()));
        }
        Ok(fresh)
    }
}

static REPO: Lazy<RwLock<Option<Arc<dyn PrometheusRepository>>>> = Lazy::new(|| RwLock::new(None));

pub fn get_repository() -> Arc<dyn PrometheusRepository> {
    if let Some(repo) = REPO.read().unwrap().as_ref() {
        return Arc::clone(repo);
    }
    // Build default repo
    match HttpPrometheusRepository::from_env() {
        Ok(http) => {
            let arc: Arc<dyn PrometheusRepository> = Arc::new(http);
            *REPO.write().unwrap() = Some(Arc::clone(&arc));
            arc
        }
        Err(err) => {
            // Fallback repo that returns the error on all calls
            struct ErrRepo {
                err: PrometheusError,
            }
            #[async_trait]
            impl PrometheusRepository for ErrRepo {
                async fn query(
                    &self,
                    _query: &str,
                    _time: Option<&str>,
                ) -> Result<PrometheusQueryResult, PrometheusError> {
                    Err(PrometheusError::ApiError(format!(
                        "Repository init error: {:?}",
                        self.err
                    )))
                }
                async fn query_range(
                    &self,
                    _query: &str,
                    _start: &str,
                    _end: &str,
                    _step: &str,
                ) -> Result<PrometheusQueryResult, PrometheusError> {
                    Err(PrometheusError::ApiError(format!(
                        "Repository init error: {:?}",
                        self.err
                    )))
                }
                async fn list_metrics(&self) -> Result<Vec<String>, PrometheusError> {
                    Err(PrometheusError::ApiError(format!(
                        "Repository init error: {:?}",
                        self.err
                    )))
                }
                async fn get_metadata(
                    &self,
                    _metric: &str,
                ) -> Result<Vec<MetricMetadata>, PrometheusError> {
                    Err(PrometheusError::ApiError(format!(
                        "Repository init error: {:?}",
                        self.err
                    )))
                }
                async fn get_series(
                    &self,
                    _match_strings: Vec<&str>,
                ) -> Result<Vec<std::collections::HashMap<String, String>>, PrometheusError>
                {
                    Err(PrometheusError::ApiError(format!(
                        "Repository init error: {:?}",
                        self.err
                    )))
                }
                async fn get_label_values(
                    &self,
                    _label_name: &str,
                ) -> Result<Vec<String>, PrometheusError> {
                    Err(PrometheusError::ApiError(format!(
                        "Repository init error: {:?}",
                        self.err
                    )))
                }
            }
            let arc: Arc<dyn PrometheusRepository> = Arc::new(ErrRepo { err });
            *REPO.write().unwrap() = Some(Arc::clone(&arc));
            arc
        }
    }
}

/// Override the repository instance (DI for tests or custom setups)
#[allow(dead_code)]
pub fn set_repository(repo: Arc<dyn PrometheusRepository>) {
    *REPO.write().unwrap() = Some(repo);
}

/// Testing-only: legacy helper; prefer set_repository
#[cfg(test)]
pub fn set_repository_for_tests(repo: Arc<dyn PrometheusRepository>) {
    *REPO.write().unwrap() = Some(repo);
}
