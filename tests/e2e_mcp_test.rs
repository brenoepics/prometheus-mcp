use serde_json::json;
use serde_json::Value;
use std::io::Write;
use std::process::{Command, Stdio};

#[tokio::test]
async fn test_mcp_server_tools_schema() {
    // Start the MCP server process
    let mut child = Command::new("target/debug/prometheus-mcp")
        .arg("--mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start MCP server");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    // Send initialize request
    let init_req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "vscode-test",
                "version": "1.0"
            }
        }
    });

    writeln!(stdin, "{}", init_req.to_string()).expect("Failed to write to stdin");

    // Send tools/list request
    let tools_req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    });

    writeln!(stdin, "{}", tools_req.to_string()).expect("Failed to write to stdin");

    // Read response line by line
    use std::io::{BufRead, BufReader};
    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);
    let mut responses = Vec::new();

    for line in reader.lines().take(2) {
        let line = line.expect("Failed to read line");
        if !line.is_empty() {
            println!("Server response: {}", line);
            let response: Value =
                serde_json::from_str(&line).expect("Failed to parse JSON response");
            responses.push(response);
        }
    }

    // Ensure we got both responses
    assert_eq!(
        responses.len(),
        2,
        "Expected initialize and tools/list responses"
    );

    // Validate tools/list response contains prometheus_get_series with correct schema
    let tools_response = &responses[1];
    let tools = tools_response["result"]["tools"]
        .as_array()
        .expect("tools should be an array");

    let series_tool = tools
        .iter()
        .find(|t| t["name"] == "prometheus_get_series")
        .expect("prometheus_get_series tool not found");

    println!("\nPrometheus Get Series Tool Schema:");
    println!("{}", serde_json::to_string_pretty(series_tool).unwrap());

    // Check parameters path since VS Code specifically looks there
    let schema = &series_tool["parameters"];
    assert!(schema.is_object(), "parameters must be an object");

    // Tool schema must be an object type with explicit type field
    assert_eq!(schema["type"], "object", "parameters type should be object");

    // match_strings property must exist and be an array type
    let match_strings = &schema["properties"]["match_strings"];
    assert!(
        match_strings.is_object(),
        "match_strings property must be an object"
    );
    assert_eq!(
        match_strings["type"], "array",
        "match_strings must have explicit array type"
    );

    // items must be present with explicit type
    let items = match_strings["items"]
        .as_object()
        .expect("items must be an object");
    assert_eq!(
        items["type"], "string",
        "items must have explicit string type"
    );

    // Required array must include match_strings
    let required = schema["required"]
        .as_array()
        .expect("required must be an array");
    assert!(
        required.contains(&json!("match_strings")),
        "match_strings must be in required array"
    );

    // minItems and additionalProperties for extra validation
    assert_eq!(
        match_strings["minItems"], 1,
        "match_strings should require at least one item"
    );
    assert_eq!(
        schema["additionalProperties"], false,
        "parameters should not allow additional properties"
    );

    // Clean up
    child.kill().expect("Failed to kill MCP server process");
}
