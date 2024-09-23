use pretty_assertions_sorted::assert_eq_sorted;
use type_generator::{QueryResult, ValueType};

#[test]
fn nested_schema_object() -> anyhow::Result<()> {
    let query = r#"
SELECT
    bar
FROM foo;
"#;

    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
DEFINE FIELD bar ON foo TYPE array<object>;
DEFINE FIELD bar.*.baz ON foo TYPE string;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [(
                "bar".into(),
                ValueType::Array(Box::new(ValueType::Object(
                    [("baz".into(), ValueType::String),].into()
                )))
            ),]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn schema_flexible_object() -> anyhow::Result<()> {
    let query = r#"
SELECT
    bar
FROM foo;
"#;

    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
DEFINE FIELD bar ON foo FLEXIBLE TYPE object;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [("bar".into(), ValueType::Any),].into()
        )))]
    );

    Ok(())
}
