use pretty_assertions_sorted::assert_eq_sorted;
use std::collections::HashMap;
use type_generator::QueryReturnType;

#[test]
fn query_with_variable() -> anyhow::Result<()> {
    let query_str = r#"
DELETE user RETURN $before;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let (return_types, _, _) =
        type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [(
                "before".into(),
                QueryReturnType::Object(HashMap::from([
                    ("id".into(), QueryReturnType::Record(vec!["user".into()])),
                    ("name".into(), QueryReturnType::String)
                ]))
            )]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn query_with_variable_with_multiple_returns() -> anyhow::Result<()> {
    let query_str = r#"
DELETE user RETURN $before.name AS alias, $before.xyz.baz AS baz
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD xyz ON user TYPE record<abc>;

DEFINE TABLE abc SCHEMAFULL;
DEFINE FIELD baz ON abc TYPE string;
"#;

    let (return_types, _, _) =
        type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [
                ("alias".into(), QueryReturnType::String),
                ("baz".into(), QueryReturnType::String),
            ]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn query_with_variable_with_multiple_returns_with_alias() -> anyhow::Result<()> {
    let query_str = r#"
DELETE user RETURN $after
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let (return_types, _, _) =
        type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [("after".into(), QueryReturnType::Null)].into()
        )))]
    );

    Ok(())
}

#[test]
fn query_with_this_field() -> anyhow::Result<()> {
    let query_str = r#"
SELECT
    name,
    $this.name AS alias
FROM user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let (return_types, _, _) =
        type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [
                ("name".into(), QueryReturnType::String),
                ("alias".into(), QueryReturnType::String),
            ]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn query_with_nested_query_parent_parameter() -> anyhow::Result<()> {
    let query_str = r#"
SELECT
    name,
    ($parent.name) AS alias
FROM user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let (return_types, _, _) =
        type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [
                ("name".into(), QueryReturnType::String),
                ("alias".into(), QueryReturnType::String),
            ]
            .into()
        )))]
    );

    Ok(())
}
