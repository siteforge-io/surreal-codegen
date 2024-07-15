use pretty_assertions_sorted::assert_eq_sorted;
use std::collections::HashMap;
use type_generator::QueryReturnType;

#[test]
fn query_with_subquery() -> anyhow::Result<()> {
    let query_str = r#"
SELECT
    name,
    (SELECT name FROM user) AS subquery,
    (DELETE user),
    (UPDATE user SET name = "John" RETURN NONE)
FROM ONLY user;
"#;
    let schema_str = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let (params, return_types, _) =
        type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        (params, return_types),
        (
            HashMap::new(),
            vec![QueryReturnType::Object(
                [
                    ("name".into(), QueryReturnType::String),
                    (
                        "subquery".into(),
                        QueryReturnType::Array(Box::new(QueryReturnType::Object(
                            [("name".to_string(), QueryReturnType::String)].into()
                        )))
                    ),
                    (
                        "(DELETE user)".into(),
                        QueryReturnType::Array(Box::new(QueryReturnType::Never))
                    ),
                    (
                        "(UPDATE user SET name = \'John\' RETURN NONE)".into(),
                        QueryReturnType::Array(Box::new(QueryReturnType::Never))
                    ),
                ]
                .into()
            ),]
        )
    );

    Ok(())
}
