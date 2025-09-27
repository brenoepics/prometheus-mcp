use crate::mcp::types::*;
use maplit::hashmap;
use rpc_router::{Handler, HandlerResult, RouterBuilder, RpcParams};
use serde::{Deserialize, Serialize};

/// register all tools to the router
pub fn register_tools(router_builder: RouterBuilder) -> RouterBuilder {
    router_builder
        .append_dyn("tools/list", tools_list.into_dyn())
        .append_dyn("get_current_time_in_city", current_time.into_dyn())
}

pub async fn tools_list(_request: Option<ListToolsRequest>) -> HandlerResult<ListToolsResult> {
    //let tools: Vec<Tool> = serde_json::from_str(include_str!("./templates/tools.json")).unwrap();
    let response = ListToolsResult {
        tools: vec![Tool {
            name: "get_current_time_in_city".to_string(),
            description: Some("Get the current time in the city".to_string()),
            input_schema: ToolInputSchema {
                type_name: "object".to_string(),
                properties: hashmap! {
                    "city".to_string() => ToolInputSchemaProperty {
                        type_name: Some("string".to_owned()),
                        description: Some("city name".to_owned()),
                        enum_values: None,
                    }
                },
                required: vec!["city".to_string()],
            },
        }],
        next_cursor: None,
    };
    Ok(response)
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct CurrentTimeRequest {
    pub city: Option<String>,
}

pub async fn current_time(_request: CurrentTimeRequest) -> HandlerResult<CallToolResult> {
    let result = format!("Now: {}!", chrono::Local::now().to_rfc2822());
    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text { text: result }],
        is_error: false,
    })
}
