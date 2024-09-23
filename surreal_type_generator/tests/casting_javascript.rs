use pretty_assertions_sorted::assert_eq_sorted;
use type_generator::{step_3_codegen::QueryResult, ValueType};

#[test]
fn casting_javascript() -> anyhow::Result<()> {
    let query = r#"
RETURN <number> function() { return 123; };
"#;

    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(return_types, vec![ValueType::Number]);

    Ok(())
}
