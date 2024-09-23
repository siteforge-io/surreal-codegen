use pretty_assertions_sorted::assert_eq_sorted;
use type_generator::{QueryResult, ValueType};

#[test]
fn field_defaults() -> anyhow::Result<()> {
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD created_at ON user TYPE datetime VALUE time::now() READONLY;
"#;

    let QueryResult { return_types, .. } = type_generator::step_3_codegen::query_to_return_type(
        r#"
CREATE user;
"#,
        schema,
    )?;

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [
                ("id".into(), ValueType::Record(vec!["user".into()])),
                ("created_at".into(), ValueType::Datetime),
            ]
            .into()
        )))]
    );
    Ok(())
}
