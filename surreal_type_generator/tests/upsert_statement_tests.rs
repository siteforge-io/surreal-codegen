use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{QueryResult, ValueType};

#[test]
fn simple_upsert_statement() -> anyhow::Result<()> {
    let query = r#"
UPSERT user CONTENT {
    name: "John Doe",
    age: 30,
};
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD age ON user TYPE number;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [
                ("id".into(), ValueType::Record(vec!["user".into()]).into()),
                ("name".into(), ValueType::String.into()),
                ("age".into(), ValueType::Number.into()),
            ]
            .into()
        )))]
    );

    Ok(())
}
