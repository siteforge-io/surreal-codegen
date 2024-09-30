use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, QueryResult};

#[test]
fn update_statement_with_set_field() -> anyhow::Result<()> {
    let query = r#"
UPDATE user:john SET name = "John";
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
                id: kind!(Record ["user"]),
            name: kind!(String)
        })])]
    );

    Ok(())
}

#[test]
fn update_return_before() -> anyhow::Result<()> {
    let query = r#"
UPDATE user:john SET baz = "bar" RETURN BEFORE;
"#;

    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD baz ON user TYPE string;
"#;
    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!(Either [
            kind!({
                id: kind!(Record ["user"]),
                name: kind!(String),
                baz: kind!(String)
            }),
            kind!(Null)
        ])])]
    );

    Ok(())
}

#[test]
fn update_return_after() -> anyhow::Result<()> {
    let query = r#"
UPDATE user:john SET baz = "bar" RETURN AFTER;
"#;

    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD baz ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            id: kind!(Record ["user"]),
            name: kind!(String),
            baz: kind!(String)
        })])]
    );

    Ok(())
}

#[test]
fn update_return_null() -> anyhow::Result<()> {
    let query = r#"
UPDATE user:john SET baz = "bar" RETURN NULL;
"#;

    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD baz ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(return_types, vec![kind!([kind!(Null)])]);

    Ok(())
}

#[test]
fn update_return_none() -> anyhow::Result<()> {
    let query = r#"
UPDATE user:john SET baz = "bar" RETURN NONE;
"#;

    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD baz ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(return_types, vec![kind!([kind!(Null)])]);

    Ok(())
}

#[test]
fn update_return_fields() -> anyhow::Result<()> {
    let query = r#"
UPDATE user:john SET baz = "bar" RETURN baz;
"#;

    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD baz ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            baz: kind!(String),
        })])]
    );

    Ok(())
}
