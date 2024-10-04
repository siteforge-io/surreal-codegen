use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, step_3_codegen::QueryResult};

#[test]
fn constant_string() -> anyhow::Result<()> {
    let query = r#"
SELECT
    "foo",
    123,
    true,
    false,
    NONE,
    NULL
FROM ONLY foo
"#;
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!({
            "123": kind!(Number),
            "false": kind!(Bool),
            "foo": kind!(String),
            "true": kind!(Bool),
            "NONE": kind!(Null),
            "NULL": kind!(Null)
        })]
    );

    Ok(())
}
