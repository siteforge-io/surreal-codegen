use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{QueryResult, ValueType};

#[test]
fn query_with_subquery() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name,
    (SELECT name FROM user) AS subquery,
    (DELETE user),
    (UPDATE user SET name = "John" RETURN NONE)
FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Option(Box::new(ValueType::Object(
            [
                ("name".into(), ValueType::String),
                (
                    "subquery".into(),
                    ValueType::Array(Box::new(ValueType::Object(
                        [("name".to_string(), ValueType::String)].into()
                    )))
                ),
                (
                    "(DELETE user)".into(),
                    ValueType::Array(Box::new(ValueType::Never))
                ),
                (
                    "(UPDATE user SET name = \'John\' RETURN NONE)".into(),
                    ValueType::Array(Box::new(ValueType::Never))
                ),
            ]
            .into()
        )))]
    );

    Ok(())
}
