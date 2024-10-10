use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, var_map, QueryResult};

#[test]
fn let_statement_parameter_infernece() -> anyhow::Result<()> {
    let query = r#"
LET $id: record<foo> = $page.id;

UPSERT ONLY $id CONTENT $page;
"#;
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    let upsert_type = kind!({
        id: kind!(Opt (kind!(Record ["foo"]))),
    });

    assert_eq_sorted!(
        variables,
        var_map! {
            page: kind!(Either [upsert_type.clone(), kind!([upsert_type])]),
        }
    );

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
