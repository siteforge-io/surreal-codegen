use pretty_assertions_sorted;
use surreal_type_generator::{kind, Kind, Literal, QueryResult};
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
        vec![kind!({
            id: kind!(Record ["baz"]),
            foo: kind!(Either [
                Kind::Literal(Literal::String("A".into())),
                Kind::Literal(Literal::Duration(Duration::new(86400, 0))),
                Kind::Literal(Literal::Number(123.into())),
                kind!([kind!(Either [
                    Kind::Literal(Literal::Number(1.into())),
                    Kind::Literal(Literal::Number(2.into()))
                ])]),
                kind!({
                    foo: kind!(Either [
                        kind!(String),
                        Kind::Literal(Literal::Number(123.into()))
                    ])
                })
            ])
        })]
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
        vec![kind!([kind!({
            foo: kind!([kind!(Null)]),
            num_list: kind!([kind!(Number)])
        })])]
    );
}
