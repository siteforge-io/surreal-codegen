use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, QueryResult};

#[test]
fn nested_schema_object() -> anyhow::Result<()> {
    let query = r#"
SELECT
    bar
FROM foo;
"#;

    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
DEFINE FIELD bar ON foo TYPE array<{
    baz: string
}>;
// DEFINE FIELD bar.*.baz ON foo TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            bar: kind!([
                kind!({
                    baz: kind!(String)
                })
            ])
        })])]
    );

    Ok(())
}

#[test]
fn schema_flexible_object() -> anyhow::Result<()> {
    let query = r#"
SELECT
    bar
FROM foo;
"#;

    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
DEFINE FIELD bar ON foo FLEXIBLE TYPE object;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            bar: kind!(Object)
        })])]
    );

    Ok(())
}
