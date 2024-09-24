use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::QueryResult;

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
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![surreal_type_generator::ValueType::Object(
            [
                (
                    "foo".to_string(),
                    surreal_type_generator::ValueType::Object(
                        [
                            ("bar".to_string(), surreal_type_generator::ValueType::Number),
                            ("baz".to_string(), surreal_type_generator::ValueType::Number),
                        ]
                        .into()
                    ),
                ),
                ("qux".to_string(), surreal_type_generator::ValueType::Number),
            ]
            .into()
        )]
    );

    Ok(())
}
