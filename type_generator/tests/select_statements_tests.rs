use pretty_assertions_sorted::assert_eq_sorted;
use std::collections::HashMap;
use surrealdb::sql::Table;
use type_generator::{step_3_outputs::CodegenInformation, QueryReturnType};

#[test]
fn query_specific_value() -> anyhow::Result<()> {
    let query_str = r#"
SELECT VALUE name FROM ONLY user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::String,]
        }
    );

    Ok(())
}

#[test]
fn validate_return_types() -> anyhow::Result<()> {
    let query_str = r#"
SELECT
    name,
    age,
    bool,
    datetime,
    duration,
    decimal,
    uuid
FROM
    user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD age ON user TYPE int;
DEFINE FIELD bool ON user TYPE bool;
DEFINE FIELD datetime ON user TYPE datetime;
DEFINE FIELD duration ON user TYPE duration;
DEFINE FIELD decimal ON user TYPE decimal;
DEFINE FIELD uuid ON user TYPE uuid;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
                [
                    ("name".into(), QueryReturnType::String.into()),
                    ("age".into(), QueryReturnType::Int.into()),
                    ("bool".into(), QueryReturnType::Bool.into()),
                    ("datetime".into(), QueryReturnType::Datetime.into()),
                    ("duration".into(), QueryReturnType::Duration.into()),
                    ("decimal".into(), QueryReturnType::Decimal.into()),
                    ("uuid".into(), QueryReturnType::Uuid.into()),
                ]
                .into()
            ))),]
        }
    );

    Ok(())
}

#[test]
fn validate_return_types_with_only_value() -> anyhow::Result<()> {
    let query_str = r#"
SELECT
    name
FROM ONLY user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Object(
                [("name".into(), QueryReturnType::String.into()),].into()
            ),]
        }
    );

    Ok(())
}

#[test]
fn validate_return_types_with_parameter_record() -> anyhow::Result<()> {
    let query_str = r#"
<record<user>> $user;

SELECT name FROM $user
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: [(
                "user".into(),
                QueryReturnType::Record([Table::from("user")].into())
            )]
            .into(),
            return_types: vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
                [("name".into(), QueryReturnType::String.into()),].into()
            ))),]
        }
    );

    Ok(())
}

#[test]
fn validate_nested_record_return_type() -> anyhow::Result<()> {
    let query_str = r#"
SELECT
    xyz.abc,
    xyz.user.xyz
FROM
    user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD xyz ON user TYPE record<xyz>;

DEFINE TABLE xyz SCHEMAFULL;
DEFINE FIELD abc ON xyz TYPE string;
DEFINE FIELD user ON xyz TYPE record<user>;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
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
        }
    );

    Ok(())
}

#[test]
fn query_with_alias_field() -> anyhow::Result<()> {
    let query_str = r#"
SELECT
    name as foo
FROM ONLY user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Object(
                [("foo".into(), QueryReturnType::String.into()),].into()
            ),]
        }
    );

    Ok(())
}

#[test]
fn query_with_alias_field_with_table() -> anyhow::Result<()> {
    let query_str = r#"
SELECT
    org.name as foo
FROM ONLY user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD org ON user TYPE record<org>;

DEFINE TABLE org SCHEMAFULL;
DEFINE FIELD name ON org TYPE string;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Object(
                [("foo".into(), QueryReturnType::String.into()),].into()
            ),]
        }
    );

    Ok(())
}

#[test]
fn query_field_with_all() -> anyhow::Result<()> {
    let query_str = r#"
SELECT * FROM ONLY user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Object(
                [
                    ("name".into(), QueryReturnType::String.into()),
                    ("id".into(), QueryReturnType::Record(vec!["user".into()]))
                ]
                .into()
            ),]
        }
    );

    Ok(())
}

#[test]
fn query_with_optional_fields() -> anyhow::Result<()> {
    let query_str = r#"
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
    let schema_str = r#"
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

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Object(
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
                                    QueryReturnType::Option(Box::new(
                                        QueryReturnType::String.into()
                                    ))
                                    .into()
                                ),
                                (
                                    "abc2".into(),
                                    QueryReturnType::Option(Box::new(
                                        QueryReturnType::String.into()
                                    ))
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
        }
    );

    Ok(())
}

#[test]
fn query_with_nested_array_string_field() -> anyhow::Result<()> {
    let query_str = r#"
SELECT
    tags.*
FROM
    post;
"#;
    let schema_str = r#"
DEFINE TABLE post SCHEMAFULL;
DEFINE FIELD tags ON post TYPE array<string>;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
                [(
                    "tags".into(),
                    QueryReturnType::Array(Box::new(QueryReturnType::String.into()))
                ),]
                .into()
            ))),]
        }
    );

    Ok(())
}

#[test]
fn query_with_array_field() -> anyhow::Result<()> {
    let query_str = r#"
SELECT
    tags
FROM
    post;
"#;
    let schema_str = r#"
DEFINE TABLE post SCHEMAFULL;
DEFINE FIELD tags ON post TYPE array<string>;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
                [(
                    "tags".into(),
                    QueryReturnType::Array(Box::new(QueryReturnType::String.into()))
                ),]
                .into()
            ))),]
        }
    );

    Ok(())
}

#[test]
fn select_specific_record() -> anyhow::Result<()> {
    let query_str = r#"
SELECT
    name
FROM
    user:john
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
                [("name".into(), QueryReturnType::String.into()),].into()
            ))),]
        }
    );

    Ok(())
}
