ping:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "ping" }' | ./target/debug/mcp-rs-demo --mcp

prompts-list:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "prompts/list" }' | ./target/debug/mcp-rs-demo --mcp

prompt-get:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "prompts/get", "params": {"name":"current_time","arguments": {"city": "hangzhou"} } }' | ./target/debug/mcp-rs-demo --mcp

tools-list:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/list" }' | ./target/debug/mcp-rs-demo --mcp

resources-list:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "resources/list" }' | ./target/debug/mcp-rs-demo --mcp

current-time:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "get_current_time_in_city", "arguments": {"city":"Hangzhou" } } }' | ./target/debug/mcp-rs-demo --mcp
