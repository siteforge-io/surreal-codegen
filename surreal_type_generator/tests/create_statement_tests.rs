use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, var_map, QueryResult};

#[test]
fn simple_create_content_query() -> anyhow::Result<()> {
    let query = r#"
CREATE user CONTENT {
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
fn create_return_none() -> anyhow::Result<()> {
    let query = r#"
CREATE foo RETURN NONE
"#;
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(return_types, vec![kind!([kind!(Null)])]);

    Ok(())
}

#[test]
fn create_return_null() -> anyhow::Result<()> {
    let query = r#"
CREATE foo RETURN NULL
"#;
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(return_types, vec![kind!([kind!(Null)])]);

    Ok(())
}

#[test]
fn create_return_before() -> anyhow::Result<()> {
    let query = r#"
CREATE foo RETURN BEFORE
"#;
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(return_types, vec![kind!([kind!(Null)])]);

    Ok(())
}

#[test]
fn create_with_set_field() -> anyhow::Result<()> {
    let query = r#"
CREATE user SET name = "John Doe"
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
fn create_statement_with_variable_inference() -> anyhow::Result<()> {
    let query = r#"
CREATE user CONTENT $user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD email ON user TYPE string;
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD opt ON user TYPE option<string>;
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    let user_vars = kind!({
        id: kind!(Opt (kind!(Record ["user"]))),
        name: kind!(String),
        email: kind!(String),
        created_at: kind!(Opt (kind!(Datetime))),
        opt: kind!(Opt (kind!(String)))
    });

    assert_eq_sorted!(
        variables,
        var_map! {
            user: kind!(Either [
                user_vars.clone(),
                kind!([user_vars.clone()]),
            ])
        }
    );

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            id: kind!(Record ["user"]),
            name: kind!(String),
            email: kind!(String),
            created_at: kind!(Datetime),
            opt: kind!(Opt (kind!(String)))
        })])]
    );

    Ok(())
}

#[test]
fn create_statement_with_value_and_default_clauses() -> anyhow::Result<()> {
    let query = r#"
CREATE user CONTENT $user"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD age ON user TYPE number DEFAULT 30;
DEFINE FIELD email ON user TYPE string VALUE string::lowercase($value);
DEFINE FIELD created_at ON user TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD updated_at ON user TYPE datetime VALUE time::now();
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    let user_vars = kind!({
        id: kind!(Opt (kind!(Record ["user"]))),
        name: kind!(String),
        age: kind!(Opt (kind!(Number))),
        email: kind!(String)
    });

    assert_eq_sorted!(
        variables,
        var_map! {
            user: kind!(Either [
                user_vars.clone(),
                kind!(Arr user_vars.clone()),
            ])
        }
    );

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            id: kind!(Record ["user"]),
            name: kind!(String),
            age: kind!(Number),
            email: kind!(String),
            created_at: kind!(Datetime),
            updated_at: kind!(Datetime)
        })])]
    );

    Ok(())
}
