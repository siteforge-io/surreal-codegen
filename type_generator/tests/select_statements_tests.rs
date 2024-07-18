use pretty_assertions_sorted::assert_eq_sorted;
use std::collections::HashMap;
use surrealdb::sql::Table;
use type_generator::{QueryResult, QueryReturnType};

#[test]
fn query_specific_value() -> anyhow::Result<()> {
    let query = r#"
SELECT VALUE name FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(return_types, vec![QueryReturnType::String]);

    Ok(())
}

#[test]
fn validate_return_types() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name,
    age,
    bool,
    datetime,
    duration,
    decimal,
    uuid,
    number
FROM
    user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD age ON user TYPE int;
DEFINE FIELD bool ON user TYPE bool;
DEFINE FIELD datetime ON user TYPE datetime;
DEFINE FIELD duration ON user TYPE duration;
DEFINE FIELD decimal ON user TYPE decimal;
DEFINE FIELD uuid ON user TYPE uuid;
DEFINE FIELD number ON user TYPE number;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [
                ("name".to_string(), QueryReturnType::String),
                ("age".to_string(), QueryReturnType::Int),
                ("bool".to_string(), QueryReturnType::Bool),
                ("datetime".to_string(), QueryReturnType::Datetime),
                ("duration".to_string(), QueryReturnType::Duration),
                ("decimal".to_string(), QueryReturnType::Decimal),
                ("uuid".to_string(), QueryReturnType::Uuid),
                ("number".to_string(), QueryReturnType::Number),
            ]
            .into()
        )))]
    );

    Ok(())
}

#[test]
fn validate_return_types_with_only_value() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name
FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Object(HashMap::from([(
            "name".to_string(),
            QueryReturnType::String
        )]))]
    );

    Ok(())
}

#[test]
fn validate_return_types_with_parameter_record() -> anyhow::Result<()> {
    let query = r#"
<record<user>> $user;

SELECT name FROM $user
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

    assert_eq_sorted!(
        variables,
        HashMap::from([(
            "user".to_string(),
            QueryReturnType::Record(vec![Table::from("user")])
        )])
    );

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [("name".to_string(), QueryReturnType::String)].into()
        )))]
    );

    Ok(())
}

#[test]
fn validate_nested_record_return_type() -> anyhow::Result<()> {
    let query = r#"
SELECT
    xyz.abc,
    xyz.user.xyz
FROM
    user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD xyz ON user TYPE record<xyz>;

DEFINE TABLE xyz SCHEMAFULL;
DEFINE FIELD abc ON xyz TYPE string;
DEFINE FIELD user ON xyz TYPE record<user>;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [(
                "xyz".into(),
                QueryReturnType::Object(
                    [
                        ("abc".into(), QueryReturnType::String.into()),
                        (
                            "user".into(),
                            QueryReturnType::Object(
                                [(
                                    "xyz".into(),
                                    QueryReturnType::Record([Table::from("xyz")].into())
                                ),]
                                .into()
                            )
                        ),
                    ]
                    .into()
                )
            ),]
            .into()
        ))),]
    );

    Ok(())
}

#[test]
fn query_with_alias_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name as foo
FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Object(
            [("foo".into(), QueryReturnType::String.into()),].into()
        ),]
    );

    Ok(())
}

#[test]
fn query_with_alias_field_with_table() -> anyhow::Result<()> {
    let query = r#"
SELECT
    org.name as foo
FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD org ON user TYPE record<org>;

DEFINE TABLE org SCHEMAFULL;
DEFINE FIELD name ON org TYPE string;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Object(
            [("foo".into(), QueryReturnType::String.into()),].into()
        ),]
    );

    Ok(())
}

#[test]
fn query_field_with_all() -> anyhow::Result<()> {
    let query = r#"
SELECT * FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Object(
            [
                ("name".to_string(), QueryReturnType::String),
                (
                    "id".to_string(),
                    QueryReturnType::Record(vec!["user".into()])
                )
            ]
            .into()
        ),]
    );

    Ok(())
}

#[test]
fn query_with_optional_fields() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name,
    num,
    bool,
    datetime,
    duration,
    decimal,
    xyz.abc,
    xyz.abc2
FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE option<string>;
DEFINE FIELD num ON user TYPE option<int>;
DEFINE FIELD bool ON user TYPE option<bool>;
DEFINE FIELD datetime ON user TYPE option<datetime>;
DEFINE FIELD duration ON user TYPE option<duration>;
DEFINE FIELD decimal ON user TYPE option<decimal>;
DEFINE FIELD xyz ON user TYPE option<record<xyz>>;

DEFINE TABLE xyz SCHEMAFULL;
DEFINE FIELD abc ON xyz TYPE option<string>;
DEFINE FIELD abc2 ON xyz TYPE option<string>;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Object(
            [
                (
                    "name".into(),
                    QueryReturnType::Option(Box::new(QueryReturnType::String.into())).into()
                ),
                (
                    "num".into(),
                    QueryReturnType::Option(Box::new(QueryReturnType::Int.into())).into()
                ),
                (
                    "bool".into(),
                    QueryReturnType::Option(Box::new(QueryReturnType::Bool.into())).into()
                ),
                (
                    "datetime".into(),
                    QueryReturnType::Option(Box::new(QueryReturnType::Datetime.into())).into()
                ),
                (
                    "duration".into(),
                    QueryReturnType::Option(Box::new(QueryReturnType::Duration.into())).into()
                ),
                (
                    "decimal".into(),
                    QueryReturnType::Option(Box::new(QueryReturnType::Decimal.into())).into()
                ),
                (
                    "xyz".into(),
                    QueryReturnType::Option(Box::new(QueryReturnType::Object(
                        [
                            (
                                "abc".into(),
                                QueryReturnType::Option(Box::new(QueryReturnType::String.into()))
                                    .into()
                            ),
                            (
                                "abc2".into(),
                                QueryReturnType::Option(Box::new(QueryReturnType::String.into()))
                                    .into()
                            ),
                        ]
                        .into()
                    )))
                    .into()
                ),
            ]
            .into()
        ),]
    );

    Ok(())
}

#[test]
fn query_with_nested_array_string_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    tags.*
FROM
    post;
"#;
    let schema = r#"
DEFINE TABLE post SCHEMAFULL;
DEFINE FIELD tags ON post TYPE array<string>;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [(
                "tags".into(),
                QueryReturnType::Array(Box::new(QueryReturnType::String.into()))
            ),]
            .into()
        ))),]
    );

    Ok(())
}

#[test]
fn query_with_array_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    tags
FROM
    post;
"#;
    let schema = r#"
DEFINE TABLE post SCHEMAFULL;
DEFINE FIELD tags ON post TYPE array<string>;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [(
                "tags".into(),
                QueryReturnType::Array(Box::new(QueryReturnType::String.into()))
            ),]
            .into()
        ))),]
    );

    Ok(())
}

#[test]
fn select_specific_record() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name
FROM
    user:john
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
            [("name".to_string(), QueryReturnType::String)].into()
        )))]
    );

    Ok(())
}

#[test]
fn query_with_object_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    xyz
FROM
    user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD xyz ON user TYPE object;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [("xyz".into(), QueryReturnType::Any.into()),].into()
        ))),]
    );

    Ok(())
}

#[test]
fn query_with_nested_object_all_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    xyz.*
FROM
    user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD xyz ON user TYPE record<xyz>;

DEFINE TABLE xyz SCHEMAFULL;
DEFINE FIELD abc ON xyz TYPE string;
DEFINE FIELD num ON xyz TYPE int;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [(
                "xyz".into(),
                QueryReturnType::Object(
                    [
                        ("id".into(), QueryReturnType::Record(vec!["xyz".into()])),
                        ("abc".into(), QueryReturnType::String.into()),
                        ("num".into(), QueryReturnType::Int.into()),
                    ]
                    .into()
                )
            ),]
            .into()
        ))),]
    );

    Ok(())
}

#[test]
fn query_with_nested_optional_object() -> anyhow::Result<()> {
    let query = r#"
SELECT
    xyz.foo
FROM
    user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD xyz ON user TYPE option<object>;
DEFINE FIELD xyz.foo ON xyz TYPE option<string>;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [(
                "xyz".into(),
                QueryReturnType::Option(Box::new(QueryReturnType::Object(
                    [(
                        "foo".into(),
                        QueryReturnType::Option(Box::new(QueryReturnType::String))
                    )]
                    .into()
                )))
            )]
            .into()
        )))]
    );

    Ok(())
}
