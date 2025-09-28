use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use prometheus_mcp::mcp::repository::PrometheusRepository;
use prometheus_mcp::mcp::tools::{
    prometheus_get_label_values, prometheus_get_metadata, prometheus_get_series,
    prometheus_list_metrics, prometheus_query, prometheus_query_range, tools_list,
    PrometheusGetLabelValuesRequest, PrometheusGetMetadataRequest, PrometheusGetSeriesRequest,
    PrometheusListMetricsRequest, PrometheusQueryRangeRequest, PrometheusQueryRequest,
};
use prometheus_mcp::mcp::types::{CallToolResult, CallToolResultContent};
use prometheus_mcp::set_repository;

use prometheus_mcp::mcp::prometheus_client::{
    MetricMetadata, PrometheusData, PrometheusQueryResult,
};

struct MockRepo;

#[async_trait]
impl PrometheusRepository for MockRepo {
    async fn query(
        &self,
        _query: &str,
        _time: Option<&str>,
    ) -> Result<PrometheusQueryResult, prometheus_mcp::mcp::prometheus_client::PrometheusError>
    {
        Ok(PrometheusQueryResult {
            status: "success".into(),
            data: PrometheusData {
                result_type: "vector".into(),
                result: vec![],
            },
        })
    }

    async fn query_range(
        &self,
        _query: &str,
        _start: &str,
        _end: &str,
        _step: &str,
    ) -> Result<PrometheusQueryResult, prometheus_mcp::mcp::prometheus_client::PrometheusError>
    {
        Ok(PrometheusQueryResult {
            status: "success".into(),
            data: PrometheusData {
                result_type: "matrix".into(),
                result: vec![],
            },
        })
    }

    async fn list_metrics(
        &self,
    ) -> Result<Vec<String>, prometheus_mcp::mcp::prometheus_client::PrometheusError> {
        Ok(vec!["up".into(), "node_cpu_seconds_total".into()])
    }

    async fn get_metadata(
        &self,
        metric: &str,
    ) -> Result<Vec<MetricMetadata>, prometheus_mcp::mcp::prometheus_client::PrometheusError> {
        Ok(vec![MetricMetadata {
            metric: metric.into(),
            type_name: "counter".into(),
            help: "help".into(),
            unit: "seconds".into(),
        }])
    }

    async fn get_series(
        &self,
        _match_strings: Vec<&str>,
    ) -> Result<Vec<HashMap<String, String>>, prometheus_mcp::mcp::prometheus_client::PrometheusError>
    {
        let mut m = HashMap::new();
        m.insert("__name__".into(), "up".into());
        Ok(vec![m])
    }

    async fn get_label_values(
        &self,
        label_name: &str,
    ) -> Result<Vec<String>, prometheus_mcp::mcp::prometheus_client::PrometheusError> {
        Ok(vec![
            format!("{}-a", label_name),
            format!("{}-b", label_name),
        ])
    }
}

fn extract_text(result: &CallToolResult) -> String {
    for c in &result.content {
        if let CallToolResultContent::Text { text } = c {
            return text.clone();
        }
    }
    String::new()
}

#[tokio::test]
async fn test_tools_with_mock_repo() {
    set_repository(Arc::new(MockRepo));

    let res = prometheus_query(PrometheusQueryRequest {
        query: "up".into(),
        time: None,
    })
    .await
    .unwrap();
    assert!(!res.is_error);

    let res = prometheus_query_range(PrometheusQueryRangeRequest {
        query: "up".into(),
        start: "0".into(),
        end: "1".into(),
        step: "1".into(),
    })
    .await
    .unwrap();
    assert!(!res.is_error);

    let res = prometheus_list_metrics(PrometheusListMetricsRequest {})
        .await
        .unwrap();
    assert!(extract_text(&res).contains("up"));

    let res = prometheus_get_metadata(PrometheusGetMetadataRequest {
        metric: "up".into(),
    })
    .await
    .unwrap();
    assert!(extract_text(&res).contains("counter"));

    let res = prometheus_get_series(PrometheusGetSeriesRequest {
        match_strings: vec!["up".into()],
    })
    .await
    .unwrap();
    assert!(extract_text(&res).contains("__name__"));

    let res = prometheus_get_label_values(PrometheusGetLabelValuesRequest {
        label_name: "job".into(),
    })
    .await
    .unwrap();
    assert!(extract_text(&res).contains("job-a"));

    // Verify tools/list includes `items` for array properties (fix for validator requiring items)
    let tools_res = tools_list(None).await.unwrap();
    let series_tool = tools_res
        .tools
        .into_iter()
        .find(|t| t.name == "prometheus_get_series")
        .expect("prometheus_get_series not found in tools list");
    let prop = series_tool
        .input_schema
        .properties
        .get("match_strings")
        .expect("match_strings property missing");
    assert_eq!(prop.type_name.as_deref(), Some("array"));
    assert!(
        prop.items.is_some(),
        "match_strings.items should be present"
    );
    let item = prop.items.as_ref().unwrap();
    assert_eq!(item.type_name.as_deref(), Some("string"));
}
