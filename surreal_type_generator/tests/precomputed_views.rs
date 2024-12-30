use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, QueryResult};

#[test]
fn precomputed_views() -> anyhow::Result<()> {
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
DEFINE FIELD num ON TABLE foo TYPE number;

DEFINE TABLE baz AS SELECT
    *
FROM foo;
"#;

    let query = r#"SELECT * FROM baz;"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            id: kind!(Record ["baz"]),
            num: kind!(Number)
        })])]
    );

    Ok(())
}

#[test]
fn precomputed_views_with_new_fields() -> anyhow::Result<()> {
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
DEFINE FIELD num ON TABLE foo TYPE number;
DEFINE FIELD beep ON TABLE foo TYPE number;

DEFINE TABLE baz AS SELECT
    *,
    5 as five
FROM foo;
"#;

    let query = r#"
SELECT num, five FROM baz;
SELECT * FROM baz;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![
            kind!([kind!({
                num: kind!(Number),
                five: kind!(Number)
            })]),
            kind!([kind!({
                id: kind!(Record ["baz"]),
                num: kind!(Number),
                five: kind!(Number),
                beep: kind!(Number)
            })])
        ]
    );

    Ok(())
}
