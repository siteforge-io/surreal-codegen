use pretty_assertions_sorted::assert_eq_sorted;
use type_generator::{QueryResult, QueryReturnType};

#[test]
fn select_group_by() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name,
    5 as baz
FROM
    user
GROUP BY
    name
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD age ON user TYPE int;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [
                ("name".to_string(), QueryReturnType::String),
                ("baz".to_string(), QueryReturnType::Number),
            ]
            .into()
        ))),]
    );

    Ok(())
}

#[test]
fn select_group_by_aggregate() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name,
    count() as total
FROM
    user
GROUP BY
    name
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
                ("name".to_string(), QueryReturnType::String),
                ("total".to_string(), QueryReturnType::Number),
            ]
            .into()
        ))),]
    );

    Ok(())
}

#[test]
fn select_group_by_group_all() -> anyhow::Result<()> {
    let query = r#"
SELECT
    count() as total
FROM
    user
GROUP ALL
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
            [("total".to_string(), QueryReturnType::Number),].into()
        ))),]
    );

    Ok(())
}
