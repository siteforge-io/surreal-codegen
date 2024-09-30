use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, QueryResult};

#[test]
fn field_defaults() -> anyhow::Result<()> {
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD created_at ON user TYPE datetime VALUE time::now() READONLY;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(
            r#"
CREATE user;
"#,
            schema,
        )?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            id: kind!(Record ["user"]),
            created_at: kind!(Datetime)
        })])]
    );
    Ok(())
}
