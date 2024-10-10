use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, QueryResult};

#[test]
fn let_statement_parameter_infernece() -> anyhow::Result<()> {
    let query = r#"
LET $id: record<foo> = $page.id;

UPSERT ONLY $id CONTENT $page;
"#;
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![
            kind!(Null),
            kind!({
                id: kind!(Record ["foo"]),
            })
        ]
    );

    Ok(())
}
