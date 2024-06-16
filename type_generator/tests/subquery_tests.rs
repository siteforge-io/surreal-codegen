use pretty_assertions_sorted::assert_eq_sorted;
use std::collections::HashMap;
use type_generator::{step_3_outputs::CodegenInformation, QueryReturnType};

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

    let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

    assert_eq_sorted!(
        codegen_info,
        CodegenInformation {
            parameters: HashMap::new(),
            return_types: vec![QueryReturnType::Object(
                [
                    ("name".into(), QueryReturnType::String.into()),
                    (
                        "subquery".into(),
                        QueryReturnType::Array(Box::new(QueryReturnType::Object(
                            [("name".into(), QueryReturnType::String.into()),].into()
                        )))
                    ),
                    (
                        "(DELETE user)".into(),
                        QueryReturnType::Array(Box::new(QueryReturnType::Never.into()))
                    ),
                    (
                        "(UPDATE user SET name = \'John\' RETURN NONE)".into(),
                        QueryReturnType::Array(Box::new(QueryReturnType::Never.into()))
                    ),
                ]
                .into()
            ),]
        }
    );

    Ok(())
}
