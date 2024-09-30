use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, step_3_codegen::QueryResult};

#[test]
fn casting_javascript() -> anyhow::Result<()> {
    let query = r#"
RETURN <number> function() { return 123; };
"#;

    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(return_types, vec![kind!(Number)]);

    Ok(())
}
