use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, QueryResult};

#[test]
fn query_with_variable() -> anyhow::Result<()> {
    let query = r#"
DELETE user RETURN $before;
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

    // $before should not be a required variable
    assert_eq_sorted!(variables, [].into());

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            before: kind!({
                id: kind!(Record ["user"]),
                name: kind!(String)
            })
        })])]
    );

    Ok(())
}

#[test]
fn query_with_variable_with_multiple_returns() -> anyhow::Result<()> {
    let query = r#"
DELETE user RETURN $before.name AS alias, $before.xyz.baz AS baz
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD xyz ON user TYPE record<abc>;

DEFINE TABLE abc SCHEMAFULL;
DEFINE FIELD baz ON abc TYPE string;
"#;

    let QueryResult {
        return_types,
        variables,
        ..
    } = surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    // $before should not be a required variable
    assert_eq_sorted!(variables, [].into());

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            alias: kind!(String),
            baz: kind!(String)
        })])]
    );

    Ok(())
}

#[test]
fn query_with_variable_with_multiple_returns_with_alias() -> anyhow::Result<()> {
    let query = r#"
DELETE user RETURN $after
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

    // $after should not be a required variable
    assert_eq_sorted!(variables, [].into());

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            after: kind!(Null)
        })])]
    );

    Ok(())
}

#[test]
fn query_with_this_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name,
    $this.name AS alias
FROM user;
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

    // $this should not be a required variable
    assert_eq_sorted!(variables, [].into());

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            name: kind!(String),
            alias: kind!(String)
        })])]
    );

    Ok(())
}

#[test]
fn query_with_nested_query_parent_parameter() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name,
    ($parent.name) AS alias
FROM user;
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

    // $parent should not be a required variable
    assert_eq_sorted!(variables, [].into());

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            name: kind!(String),
            alias: kind!(String)
        })])]
    );

    Ok(())
}
