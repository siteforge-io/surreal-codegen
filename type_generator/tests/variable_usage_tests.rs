use pretty_assertions_sorted::assert_eq_sorted;
use std::collections::HashMap;
use type_generator::{QueryResult, ValueType};

#[test]
fn query_with_variable() -> anyhow::Result<()> {
    let query = r#"
DELETE user RETURN $before;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    // $before should not be a required variable
    assert_eq_sorted!(variables, HashMap::from([]));

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [(
                "before".into(),
                ValueType::Object(HashMap::from([
                    ("id".into(), ValueType::Record(vec!["user".into()])),
                    ("name".into(), ValueType::String)
                ]))
            )]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn query_with_variable_with_multiple_returns() -> anyhow::Result<()> {
    let query = r#"
DELETE user RETURN $before.name AS alias, $before.xyz.baz AS baz
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD xyz ON user TYPE record<abc>;

DEFINE TABLE abc SCHEMAFULL;
DEFINE FIELD baz ON abc TYPE string;
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    // $before should not be a required variable
    assert_eq_sorted!(variables, HashMap::from([]));

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [
                ("alias".into(), ValueType::String),
                ("baz".into(), ValueType::String),
            ]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn query_with_variable_with_multiple_returns_with_alias() -> anyhow::Result<()> {
    let query = r#"
DELETE user RETURN $after
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    // $after should not be a required variable
    assert_eq_sorted!(variables, HashMap::from([]));

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [("after".into(), ValueType::Null)].into()
        )))]
    );

    Ok(())
}

#[test]
fn query_with_this_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name,
    $this.name AS alias
FROM user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    // $this should not be a required variable
    assert_eq_sorted!(variables, HashMap::from([]));

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [
                ("name".into(), ValueType::String),
                ("alias".into(), ValueType::String),
            ]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn query_with_nested_query_parent_parameter() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name,
    ($parent.name) AS alias
FROM user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    // $parent should not be a required variable
    assert_eq_sorted!(variables, HashMap::from([]));

    assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [
                ("name".into(), ValueType::String),
                ("alias".into(), ValueType::String),
            ]
            .into()
        )))]
    );

    Ok(())
}
