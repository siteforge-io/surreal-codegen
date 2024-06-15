use pretty_assertions_sorted::assert_eq_sorted;
use std::collections::HashMap;
use type_generator::{step_3_outputs::CodegenInformation, QueryReturnType};

#[test]
fn query_with_simple_delete() -> anyhow::Result<()> {
    let query_str = r#"
DELETE FROM user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Array(Box::new(
                QueryReturnType::Never.into()
            )),]
        }
    );

    Ok(())
}

#[test]
fn query_with_delete_with_only() -> anyhow::Result<()> {
    let query_str = r#"
DELETE FROM ONLY user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Never,]
        }
    );

    Ok(())
}

#[test]
fn query_with_delete_with_after_output() -> anyhow::Result<()> {
    let query_str = r#"
DELETE user RETURN AFTER;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
"#;

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Array(Box::new(
                QueryReturnType::Null.into()
            )),]
        }
    );

    Ok(())
}

#[test]
fn query_with_delete_with_before_output() -> anyhow::Result<()> {
    let query_str = r#"
DELETE user RETURN BEFORE;
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

#[test]
fn query_with_delete_return_fields() -> anyhow::Result<()> {
    let query_str = r#"
DELETE user RETURN name;
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
                [("name".into(), QueryReturnType::Null.into()),].into()
            ))),]
        }
    );

    Ok(())
}
