use crate::mcp::types::*;
use rpc_router::HandlerResult;

/// No-op handler for `resources/list` to satisfy clients that probe this method.
pub async fn compat_resources_list(
    _request: Option<ListResourcesRequest>,
) -> HandlerResult<ListResourcesResult> {
    Ok(ListResourcesResult {
        resources: vec![],
        next_cursor: None,
    })
}

/// No-op handler for `resources/templates/list` to satisfy clients that probe this method.
pub async fn compat_resource_templates_list(
    _request: Option<ListResourceTemplatesRequest>,
) -> HandlerResult<ListResourceTemplatesResult> {
    Ok(ListResourceTemplatesResult {
        resource_templates: vec![],
        next_cursor: None,
    })
}

/// No-op handler for `prompts/list` to satisfy clients that probe this method.
pub async fn compat_prompts_list(
    _request: Option<ListPromptsRequest>,
) -> HandlerResult<ListPromptsResult> {
    Ok(ListPromptsResult {
        prompts: vec![],
        next_cursor: None,
    })
}
