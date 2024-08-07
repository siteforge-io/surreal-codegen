use pretty_assertions_sorted::assert_eq_sorted;
use type_generator::{QueryResult, ValueType};

#[test]
fn precomputed_views() -> anyhow::Result<()> {
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
DEFINE FIELD num ON TABLE foo TYPE number;

DEFINE TABLE baz AS SELECT
    *
FROM foo;
"#;

    let query = r#"SELECT * FROM baz;"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [
                (
                    "id".to_string(),
                    ValueType::Record(vec!["baz".into()])
                ),
                ("num".to_string(), ValueType::Number),
            ]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn precomputed_views_with_new_fields() -> anyhow::Result<()> {
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
DEFINE FIELD num ON TABLE foo TYPE number;
DEFINE FIELD beep ON TABLE foo TYPE number;

DEFINE TABLE baz AS SELECT
    *,
    5 as five
FROM foo;
"#;

    let query = r#"
SELECT num, five FROM baz;
SELECT * FROM baz;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![
            ValueType::Array(Box::new(ValueType::Object(
                [
                    ("num".to_string(), ValueType::Number),
                    ("five".to_string(), ValueType::Number),
                ]
                .into()
            ))),
            ValueType::Array(Box::new(ValueType::Object(
                [
                    (
                        "id".to_string(),
                        ValueType::Record(vec!["baz".into()])
                    ),
                    ("num".to_string(), ValueType::Number),
                    ("five".to_string(), ValueType::Number),
                    ("beep".to_string(), ValueType::Number),
                ]
                .into()
            ))),
        ]
    );

    Ok(())
}
