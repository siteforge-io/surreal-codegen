use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{QueryResult, ValueType};

#[test]
fn insert_single_record() -> anyhow::Result<()> {
    let query = r#"
INSERT INTO user $user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    let user_vars = ValueType::Object(
        [
            (
                "id".to_string(),
                ValueType::Option(Box::new(ValueType::Record(vec!["user".into()]))),
            ),
            ("name".to_string(), ValueType::String),
        ]
        .into(),
    );

    assert_eq_sorted!(
        variables,
        [(
            "user".to_string(),
            ValueType::Either(vec![
                ValueType::Array(Box::new(user_vars.clone())),
                user_vars.clone(),
            ])
        )]
        .into()
    );

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [
                ("id".into(), ValueType::Record(vec!["user".into()])),
                ("name".into(), ValueType::String),
            ]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn insert_multiple_records() -> anyhow::Result<()> {
    let query = r#"
INSERT INTO user $users;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [
                ("id".into(), ValueType::Record(vec!["user".into()])),
                ("name".into(), ValueType::String),
            ]
            .into()
        )))]
    );

    Ok(())
}
