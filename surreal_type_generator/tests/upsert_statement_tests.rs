use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, var_map, QueryResult};

#[test]
fn simple_upsert_statement() -> anyhow::Result<()> {
    let query = r#"
UPSERT user CONTENT {
    name: "John Doe",
    age: 30,
};
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD age ON user TYPE number;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            id: kind!(Record ["user"]),
            name: kind!(String),
            age: kind!(Number)
        })])]
    );

    Ok(())
}

#[test]
fn id_upsert_statement() -> anyhow::Result<()> {
    let query = r#"
UPSERT user:john CONTENT {
    name: "John Doe",
    age: 30,
};
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD age ON user TYPE number;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            id: kind!(Record ["user"]),
            name: kind!(String),
            age: kind!(Number)
        })])]
    );

    Ok(())
}

#[test]
fn param_upsert_statement() -> anyhow::Result<()> {
    let query = r#"
<record<user>> $id;

UPSERT $id CONTENT $content;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD age ON user TYPE number;
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    let user_upsert = kind!({
        id: kind!(Opt (kind!(Record ["user"]))),
        name: kind!(String),
        age: kind!(Number),
    });

    assert_eq_sorted!(
        variables,
        var_map! {
            id: kind!(Record ["user"]),
            content: kind!(Either [user_upsert.clone(), kind!([user_upsert])])
        }
    );

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            id: kind!(Record ["user"]),
            name: kind!(String),
            age: kind!(Number)
        })])]
    );

    Ok(())
}
