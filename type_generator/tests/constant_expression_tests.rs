use pretty_assertions_sorted::assert_eq_sorted;
use std::collections::HashMap;
use type_generator::{step_3_codegen::QueryResult, ValueType};

#[test]
fn constant_string() -> anyhow::Result<()> {
    let query = r#"
SELECT
    "foo",
    123,
    true,
    false,
    NONE,
    NULL
FROM ONLY foo
"#;
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Option(Box::new(ValueType::Object(
            HashMap::from([
                ("foo".to_string(), ValueType::String),
                ("123".to_string(), ValueType::Number),
                ("true".to_string(), ValueType::Bool),
                ("false".to_string(), ValueType::Bool),
                ("NONE".to_string(), ValueType::Null),
                ("NULL".to_string(), ValueType::Null),
            ])
        )))]
    );
    Ok(())
}
