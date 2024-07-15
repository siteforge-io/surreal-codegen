use pretty_assertions_sorted::assert_eq_sorted;
use std::collections::HashMap;
use type_generator::QueryReturnType;

#[test]
fn constant_string() -> anyhow::Result<()> {
    let query_str = r#"
SELECT
    "foo",
    123,
    true,
    false,
    NONE,
    NULL
FROM ONLY foo
"#;
    let schema_str = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let (params, return_types, _) =
        type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        (params, return_types),
        (
            HashMap::new(),
            vec![QueryReturnType::Object(HashMap::from([
                ("foo".to_string(), QueryReturnType::String),
                ("123".to_string(), QueryReturnType::Number),
                ("true".to_string(), QueryReturnType::Bool),
                ("false".to_string(), QueryReturnType::Bool),
                ("NONE".to_string(), QueryReturnType::Null),
                ("NULL".to_string(), QueryReturnType::Null),
            ]))]
        )
    );

    Ok(())
}
