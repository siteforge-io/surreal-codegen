use pretty_assertions_sorted::assert_eq_sorted;
use type_generator::QueryReturnType;

#[test]
fn query_with_simple_delete() -> anyhow::Result<()> {
    let query_str = r#"
DELETE FROM user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
"#;

    let (return_types, _, _) =
        type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Never))]
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

    let (return_types, _, _) =
        type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(return_types, vec![QueryReturnType::Never]);

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

    let (return_types, _, _) =
        type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Null))]
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

    let (return_types, _, _) =
        type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

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
        ))),]
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

    let (return_types, _, _) =
        type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        return_types,
        vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
            [("name".to_string(), QueryReturnType::Null)].into()
        )))]
    );

    Ok(())
}
