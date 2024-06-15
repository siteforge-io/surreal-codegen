use pretty_assertions_sorted::assert_eq_sorted;
use std::collections::HashMap;
use type_generator::{step_3_outputs::CodegenInformation, QueryReturnType};

#[test]
fn query_with_variable() -> anyhow::Result<()> {
    let query_str = r#"
DELETE user RETURN $before;
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
                [(
                    "before".into(),
                    QueryReturnType::Object(HashMap::from([(
                        "name".into(),
                        QueryReturnType::String
                    )]))
                )]
                .into()
            )))]
        }
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

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
                [
                    ("alias".into(), QueryReturnType::String),
                    ("baz".into(), QueryReturnType::String),
                ]
                .into()
            )))]
        }
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

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
                [("after".into(), QueryReturnType::Null)].into()
            )))]
        }
    );

    Ok(())
}
