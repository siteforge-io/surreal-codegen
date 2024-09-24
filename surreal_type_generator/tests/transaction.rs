use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{step_3_codegen::QueryResult, ValueType};

#[test]
fn transaction_return_type() -> anyhow::Result<()> {
    let query = r#"
BEGIN;

RETURN true;

COMMIT;
"#;

    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(return_types, vec![ValueType::Bool]);

    Ok(())
}
