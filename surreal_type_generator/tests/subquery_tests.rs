use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, QueryResult};

#[test]
fn query_with_subquery() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name,
    (SELECT name FROM user) AS subquery,
    (DELETE user),
    (UPDATE user SET name = "John" RETURN NONE)
FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!({
            "name": kind!(String),
            "subquery": kind!([kind!({
                "name": kind!(String)
            })]),
            "(DELETE user)": kind!([kind!(Null)]),
            "(UPDATE user SET name = \'John\' RETURN NONE)": kind!([kind!(Null)])
        })]
    );

    Ok(())
}
