use prometheus_mcp::mcp::tools::tools_list;

#[tokio::test]
async fn test_tool_schemas_are_well_formed() {
    let res = tools_list(None).await.expect("tools/list failed");
    assert!(!res.tools.is_empty(), "No tools returned");

    for tool in res.tools {
        // Input schema should be an object with properties
        assert_eq!(
            tool.input_schema.type_name, "object",
            "tool {} input_schema.type must be object",
            tool.name
        );

        for (prop_name, prop) in tool.input_schema.properties.iter() {
            // Each property should have a type
            assert!(
                prop.type_name.is_some(),
                "tool {} property '{}' must have a type",
                tool.name,
                prop_name
            );

            if prop.type_name.as_deref() == Some("array") {
                // Array-typed properties must define items
                assert!(
                    prop.items.is_some(),
                    "tool {} property '{}' is an array and must have 'items' schema",
                    tool.name,
                    prop_name
                );
                let items = prop.items.as_ref().unwrap();
                assert!(
                    items.type_name.is_some(),
                    "tool {} property '{}' items must specify a type",
                    tool.name,
                    prop_name
                );
            }
        }
    }
}
