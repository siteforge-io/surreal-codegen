use pretty_assertions_sorted::assert_eq_sorted;
use type_generator::QueryResult;

#[test]
fn can_interpret_object() -> Result<(), anyhow::Error> {
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let query = r#"
RETURN {
    foo: {
        bar: 1,
        baz: 2,
    },
    qux: 3,
};
"#;

    let QueryResult { return_types, .. } =
        type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![type_generator::QueryReturnType::Object(
            [
                (
                    "foo".to_string(),
                    type_generator::QueryReturnType::Object(
                        [
                            ("bar".to_string(), type_generator::QueryReturnType::Number),
                            ("baz".to_string(), type_generator::QueryReturnType::Number),
                        ]
                        .into()
                    ),
                ),
                ("qux".to_string(), type_generator::QueryReturnType::Number),
            ]
            .into()
        )]
    );

    Ok(())
}
