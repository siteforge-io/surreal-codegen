use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, var_map, QueryResult};

#[test]
fn insert_single_record() -> anyhow::Result<()> {
    let query = r#"
INSERT INTO user $user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    let user_vars = kind!({
        id: kind!(Opt (kind!(Record ["user"]))),
        name: kind!(String)
    });

    assert_eq_sorted!(
        variables,
        var_map! {
            user: kind!(Either [
                kind!(Arr user_vars.clone()),
                user_vars.clone()
            ])
        }
    );

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
fn insert_multiple_records() -> anyhow::Result<()> {
    let query = r#"
INSERT INTO user $users;
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
