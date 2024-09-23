use pretty_assertions_sorted::assert_eq_sorted;
use type_generator::{QueryResult, ValueType};

#[test]
fn simple_create_content_query() -> anyhow::Result<()> {
    let query = r#"
CREATE user CONTENT {
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
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

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

#[test]
fn create_return_none() -> anyhow::Result<()> {
    let query = r#"
CREATE foo RETURN NONE
"#;
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Never))]
    );

    Ok(())
}

#[test]
fn create_return_null() -> anyhow::Result<()> {
    let query = r#"
CREATE foo RETURN NULL
"#;
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Null))]
    );

    Ok(())
}

#[test]
fn create_return_before() -> anyhow::Result<()> {
    let query = r#"
CREATE foo RETURN BEFORE
"#;
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Null))]
    );

    Ok(())
}

#[test]
fn create_with_set_field() -> anyhow::Result<()> {
    let query = r#"
CREATE user SET name = "John Doe"
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [
                ("id".into(), ValueType::Record(vec!["user".into()]).into()),
                ("name".into(), ValueType::String.into()),
            ]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn create_statement_with_variable_inference() -> anyhow::Result<()> {
    let query = r#"
CREATE user CONTENT $user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD email ON user TYPE string;
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD opt ON user TYPE option<string>;
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    let user_vars = ValueType::Object(
        [
            (
                "id".to_string(),
                ValueType::Option(Box::new(ValueType::Record(vec!["user".into()]))),
            ),
            ("name".to_string(), ValueType::String),
            ("email".to_string(), ValueType::String),
            (
                "created_at".to_string(),
                ValueType::Option(Box::new(ValueType::Datetime)),
            ),
            (
                "opt".to_string(),
                ValueType::Option(Box::new(ValueType::String)),
            ),
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
                ("id".into(), ValueType::Record(vec!["user".into()]).into()),
                ("name".into(), ValueType::String.into()),
                ("email".into(), ValueType::String.into()),
                ("created_at".into(), ValueType::Datetime.into()),
                (
                    "opt".into(),
                    ValueType::Option(Box::new(ValueType::String.into())).into()
                ),
            ]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn create_statement_with_value_and_default_clauses() -> anyhow::Result<()> {
    let query = r#"
CREATE user CONTENT $user"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD age ON user TYPE number DEFAULT 30;
DEFINE FIELD email ON user TYPE string VALUE string::lowercase($value);
DEFINE FIELD created_at ON user TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD updated_at ON user TYPE datetime VALUE time::now();
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    let user_vars = ValueType::Object(
        [
            (
                "id".to_string(),
                ValueType::Option(Box::new(ValueType::Record(vec!["user".into()]))),
            ),
            ("name".to_string(), ValueType::String),
            (
                "age".to_string(),
                ValueType::Option(Box::new(ValueType::Number)),
            ),
            ("email".to_string(), ValueType::String),
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
                ("age".into(), ValueType::Number),
                ("email".into(), ValueType::String),
                ("created_at".into(), ValueType::Datetime),
                ("updated_at".into(), ValueType::Datetime),
            ]
            .into()
        )))]
    );

    Ok(())
}
