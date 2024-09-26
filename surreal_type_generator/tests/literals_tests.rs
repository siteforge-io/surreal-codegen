use pretty_assertions_sorted;
use surreal_type_generator::{QueryResult, ValueType};
use surrealdb::sql::Duration;

#[test]
fn literal_types() -> anyhow::Result<()> {
    let schema = r#"
DEFINE TABLE baz SCHEMAFULL;
DEFINE FIELD foo ON TABLE baz TYPE "A" | 1d | 123 | array<1 | 2> | { foo: string | 123 };
"#;

    let query = r#"
CREATE ONLY baz CONTENT {
    foo: "A"
}"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    pretty_assertions_sorted::assert_eq_sorted!(
        return_types,
        vec![ValueType::Object(
            [
                ("id".into(), ValueType::Record(["baz".into()].into())),
                (
                    "foo".into(),
                    ValueType::Either(
                        [
                            ValueType::StringLiteral("A".into()),
                            ValueType::DurationLiteral(Duration::new(86400, 0)),
                            ValueType::NumberLiteral(123.into()),
                            ValueType::Array(Box::new(ValueType::Either(
                                [
                                    ValueType::NumberLiteral(1.into()),
                                    ValueType::NumberLiteral(2.into()),
                                ]
                                .into()
                            ))),
                            ValueType::Object(
                                [(
                                    "foo".into(),
                                    ValueType::Either(
                                        [ValueType::String, ValueType::NumberLiteral(123.into()),]
                                            .into()
                                    )
                                )]
                                .into()
                            )
                        ]
                        .into()
                    )
                )
            ]
            .into()
        )]
    );

    Ok(())
}

#[test]
fn literal_values_in_query() {
    let schema = r#"
DEFINE TABLE baz SCHEMAFULL;
"#;
    let query = r#"
SELECT [] as foo, [1, 2, 3] as num_list FROM baz;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema).unwrap();

    pretty_assertions_sorted::assert_eq_sorted!(
        return_types,
        vec![ValueType::Array(Box::new(ValueType::Object(
            [
                ("foo".into(), ValueType::Array(Box::new(ValueType::Never))),
                (
                    "num_list".into(),
                    ValueType::Array(Box::new(ValueType::Number))
                ),
            ]
            .into()
        )))]
    );
}
