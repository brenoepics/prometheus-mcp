use crate::mcp::types::*;
use rpc_router::{HandlerResult, IntoHandlerError};
use serde_json::json;

pub async fn prompts_list(
    _request: Option<ListPromptsRequest>,
) -> HandlerResult<ListPromptsResult> {
    //let prompts: Vec<Prompt> = serde_json::from_str(include_str!("./templates/prompts.json")).unwrap();
    let response = ListPromptsResult {
        next_cursor: None,
        prompts: vec![
            Prompt {
                name: "current_time".to_string(),
                description: Some("Display current time in the city".to_string()),
                arguments: Some(vec![PromptArgument {
                    name: "city".to_string(),
                    description: Some("city name".to_string()),
                    required: Some(true),
                }]),
            },
            Prompt {
                name: "analyze-code".to_string(),
                description: Some("Analyze code for potential improvements".to_string()),
                arguments: Some(vec![PromptArgument {
                    name: "language".to_string(),
                    description: Some("Programming language".to_string()),
                    required: Some(true),
                }]),
            },
        ],
    };
    Ok(response)
}

pub async fn prompts_get(request: GetPromptRequest) -> HandlerResult<PromptResult> {
    let response = match request.name.as_str() {
        "current_time" => {
            let city = request
                .arguments
                .as_ref()
                .and_then(|m| m.get("city"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| json!({"code": -32602, "message": "Missing required argument: city"}).into_handler_error())?;

            PromptResult {
                description: "Get the current time in city".to_string(),
                messages: Some(vec![PromptMessage {
                    role: "user".to_string(),
                    content: PromptMessageContent {
                        type_name: "text".to_string(),
                        text: format!("What's the time of {}?", city),
                    },
                }]),
            }
        }
        _ => {
            return Err(json!({"code": -32602, "message": "Prompt not found"}).into_handler_error())
        }
    };
    Ok(response)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_prompts_list() {
        let res = super::prompts_list(None).await.unwrap();
        assert!(!res.prompts.is_empty());
    }

    #[tokio::test]
    async fn test_prompts_get_missing_arg() {
        let req = GetPromptRequest { name: "current_time".into(), arguments: None };
        let err = super::prompts_get(req).await.err();
        assert!(err.is_some());
    }
}
