use crate::mcp::types::*;
use rpc_router::{HandlerResult, IntoHandlerError};
use serde_json::json;
use url::Url;

pub async fn resources_list(
    _request: Option<ListResourcesRequest>,
) -> HandlerResult<ListResourcesResult> {
    //let resources: Vec<Resource> = serde_json::from_str(include_str!("./templates/resources.json")).unwrap();
    let uri = Url::parse("file:///logs/app.log").map_err(|e| json!({"code": -32602, "message": format!("Invalid resource URI: {}", e)}).into_handler_error())?;
    let response = ListResourcesResult {
        resources: vec![Resource {
            uri,
            name: "Application Logs".to_string(),
            description: None,
            mime_type: Some("text/plain".to_string()),
        }],
        next_cursor: None,
    };
    Ok(response)
}

pub async fn resource_read(request: ReadResourceRequest) -> HandlerResult<ReadResourceResult> {
    let response = ReadResourceResult {
        content: ResourceContent {
            uri: request.uri.clone(),
            mime_type: Some("text/plain".to_string()),
            text: Some("2024-11-28T08:19:18.974368Z,INFO,main,this is message".to_string()),
            blob: None,
        },
    };
    Ok(response)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_resources_list() {
        let res = super::resources_list(None).await.unwrap();
        assert_eq!(res.resources.len(), 1);
    }

    #[tokio::test]
    async fn test_resource_read() {
        let uri = url::Url::parse("file:///logs/app.log").unwrap();
        let res = super::resource_read(ReadResourceRequest { uri, meta: None }).await.unwrap();
        assert!(res.content.text.is_some());
    }
}
