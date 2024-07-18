use pretty_assertions_sorted::assert_eq_sorted;
use type_generator::{QueryResult, QueryReturnType};

#[test]
fn update_statement_with_set_field() -> anyhow::Result<()> {
    let query = r#"
UPDATE user:john SET name = "John";
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [
                ("id".into(), QueryReturnType::Record(vec!["user".into()])),
                ("name".into(), QueryReturnType::String),
            ]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn update_return_before() -> anyhow::Result<()> {
    let query = r#"
UPDATE user:john SET baz = "bar" RETURN BEFORE;
"#;

    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD baz ON user TYPE string;
"#;
    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Either(
            vec![
                QueryReturnType::Object(
                    [
                        ("id".into(), QueryReturnType::Record(vec!["user".into()])),
                        ("name".into(), QueryReturnType::String),
                        ("baz".into(), QueryReturnType::String),
                    ]
                    .into()
                ),
                QueryReturnType::Null,
            ]
        )))]
    );

    Ok(())
}

#[test]
fn update_return_after() -> anyhow::Result<()> {
    let query = r#"
UPDATE user:john SET baz = "bar" RETURN AFTER;
"#;

    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD baz ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [
                ("id".into(), QueryReturnType::Record(vec!["user".into()])),
                ("name".into(), QueryReturnType::String),
                ("baz".into(), QueryReturnType::String),
            ]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn update_return_null() -> anyhow::Result<()> {
    let query = r#"
UPDATE user:john SET baz = "bar" RETURN NULL;
"#;

    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD baz ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Null))]
    );

    Ok(())
}

#[test]
fn update_return_none() -> anyhow::Result<()> {
    let query = r#"
UPDATE user:john SET baz = "bar" RETURN NONE;
"#;

    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD baz ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Never))]
    );

    Ok(())
}

#[test]
fn update_return_fields() -> anyhow::Result<()> {
    let query = r#"
UPDATE user:john SET baz = "bar" RETURN baz;
"#;

    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD baz ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [("baz".to_string(), QueryReturnType::String)].into()
        )))]
    );

    Ok(())
}
