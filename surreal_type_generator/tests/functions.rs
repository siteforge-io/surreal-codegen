use pretty_assertions_sorted;
use surreal_type_generator::QueryResult;

#[test]
fn custom_function_return_types() -> anyhow::Result<()> {
    let schema = r#"
DEFINE FUNCTION fn::foo($bar: number) {
    RETURN 5;
};"#;

    let query = r#"
RETURN fn::foo(9);
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    pretty_assertions_sorted::assert_eq_sorted!(
        return_types,
        vec![surreal_type_generator::ValueType::Number]
    );

    Ok(())
}
