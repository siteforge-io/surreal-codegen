use pretty_assertions_sorted::assert_eq_sorted;
use type_generator::{QueryResult, QueryReturnType};

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
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [
                (
                    "id".into(),
                    QueryReturnType::Record(vec!["user".into()]).into()
                ),
                ("name".into(), QueryReturnType::String.into()),
                ("age".into(), QueryReturnType::Number.into()),
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
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Never))]
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
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Null))]
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
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Null))]
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
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [
                (
                    "id".into(),
                    QueryReturnType::Record(vec!["user".into()]).into()
                ),
                ("name".into(), QueryReturnType::String.into()),
            ]
            .into()
        )))]
    );

    Ok(())
}
