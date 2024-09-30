use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, var_map, QueryResult};

#[test]
fn query_specific_value() -> anyhow::Result<()> {
    let query = r#"
SELECT VALUE name FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(return_types, vec![kind!(Opt(kind!(String)))]);

    Ok(())
}

#[test]
fn validate_return_types() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name,
    age,
    bool,
    datetime,
    duration,
    decimal,
    uuid,
    number
FROM
    user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD age ON user TYPE int;
DEFINE FIELD bool ON user TYPE bool;
DEFINE FIELD datetime ON user TYPE datetime;
DEFINE FIELD duration ON user TYPE duration;
DEFINE FIELD decimal ON user TYPE decimal;
DEFINE FIELD uuid ON user TYPE uuid;
DEFINE FIELD number ON user TYPE number;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            name: kind!(String),
            age: kind!(Int),
            bool: kind!(Bool),
            datetime: kind!(Datetime),
            duration: kind!(Duration),
            decimal: kind!(Decimal),
            uuid: kind!(Uuid),
            number: kind!(Number)
        })])]
    );

    Ok(())
}

#[test]
fn validate_return_types_with_only_value() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name
FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!(Opt(kind!({
            name: kind!(String)
        })))]
    );

    Ok(())
}

#[test]
fn validate_return_types_with_parameter_record() -> anyhow::Result<()> {
    let query = r#"
<record<user>> $user;

SELECT name FROM $user
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

    assert_eq_sorted!(
        variables,
        var_map! {
            user: kind!(Record ["user"])
        }
    );

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            name: kind!(String)
        })])]
    );

    Ok(())
}

#[test]
fn validate_nested_record_return_type() -> anyhow::Result<()> {
    let query = r#"
SELECT
    xyz.abc,
    xyz.user.xyz
FROM
    user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD xyz ON user TYPE record<xyz>;

DEFINE TABLE xyz SCHEMAFULL;
DEFINE FIELD abc ON xyz TYPE string;
DEFINE FIELD user ON xyz TYPE record<user>;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            xyz: kind!({
                abc: kind!(String),
                user: kind!({
                    xyz: kind!(Record ["xyz"])
                })
            })
        })])],
    );

    Ok(())
}

#[test]
fn query_with_alias_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name as foo
FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!(Opt(kind!({
            foo: kind!(String)
        })))]
    );

    Ok(())
}

#[test]
fn query_with_alias_field_with_table() -> anyhow::Result<()> {
    let query = r#"
SELECT
    org.name as foo
FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD org ON user TYPE record<org>;

DEFINE TABLE org SCHEMAFULL;
DEFINE FIELD name ON org TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!(Opt(kind!({
            foo: kind!(String)
        })))]
    );

    Ok(())
}

#[test]
fn query_field_with_all() -> anyhow::Result<()> {
    let query = r#"
SELECT * FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!(Opt(kind!({
            name: kind!(String),
            id: kind!(Record ["user"])
        })))]
    );

    Ok(())
}

#[test]
fn query_with_optional_fields() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name,
    num,
    bool,
    datetime,
    duration,
    decimal,
    xyz.abc,
    xyz.abc2
FROM ONLY user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE option<string>;
DEFINE FIELD num ON user TYPE option<int>;
DEFINE FIELD bool ON user TYPE option<bool>;
DEFINE FIELD datetime ON user TYPE option<datetime>;
DEFINE FIELD duration ON user TYPE option<duration>;
DEFINE FIELD decimal ON user TYPE option<decimal>;
DEFINE FIELD xyz ON user TYPE option<record<xyz>>;

DEFINE TABLE xyz SCHEMAFULL;
DEFINE FIELD abc ON xyz TYPE option<string>;
DEFINE FIELD abc2 ON xyz TYPE option<string>;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!(Opt(kind!({
            name: kind!(Opt(kind!(String))),
            num: kind!(Opt(kind!(Int))),
            bool: kind!(Opt(kind!(Bool))),
            datetime: kind!(Opt(kind!(Datetime))),
            duration: kind!(Opt(kind!(Duration))),
            decimal: kind!(Opt(kind!(Decimal))),
            xyz: kind!(Opt(kind!({
                abc: kind!(Opt(kind!(String))),
                abc2: kind!(Opt(kind!(String)))
            })))
        })))]
    );

    Ok(())
}

#[test]
fn query_with_nested_array_string_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    tags.*
FROM
    post;
"#;
    let schema = r#"
DEFINE TABLE post SCHEMAFULL;
DEFINE FIELD tags ON post TYPE array<string>;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            tags: kind!([kind!(String)])
        })])]
    );

    Ok(())
}

#[test]
fn query_with_array_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    tags,
    xyz_list as xyzs
FROM
    post;
"#;
    let schema = r#"
DEFINE TABLE xyz SCHEMAFULL;
DEFINE TABLE post SCHEMAFULL;
DEFINE FIELD tags ON post TYPE array<string>;
DEFINE FIELD xyz_list ON post TYPE array<record<xyz>> DEFAULT [];
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            tags: kind!([kind!(String)]),
            xyzs: kind!([kind!(Record ["xyz"])])
        })])]
    );

    Ok(())
}

#[test]
fn select_specific_record() -> anyhow::Result<()> {
    let query = r#"
SELECT
    name
FROM
    user:john
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
            name: kind!(String)
        })])]
    );

    Ok(())
}

#[test]
fn query_with_object_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    xyz
FROM
    user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD xyz ON user TYPE object;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            xyz: kind!(Object)
        })])]
    );

    Ok(())
}

#[test]
fn query_with_nested_object_all_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    xyz.*
FROM
    user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD xyz ON user TYPE record<xyz>;

DEFINE TABLE xyz SCHEMAFULL;
DEFINE FIELD abc ON xyz TYPE string;
DEFINE FIELD num ON xyz TYPE int;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            xyz: kind!({
                id: kind!(Record ["xyz"]),
                abc: kind!(String),
                num: kind!(Int)
            })
        })])]
    );

    Ok(())
}

#[test]
fn query_with_nested_optional_object() -> anyhow::Result<()> {
    let query = r#"
SELECT
    id,
    xyz.foo,
    xyz.abc
FROM
    user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD xyz ON user TYPE option<{
    foo: option<string>,
    abc: option<string>
}>;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            id: kind!(Record ["user"]),
            xyz: kind!(Opt(kind!({
                foo: kind!(Opt(kind!(String))),
                abc: kind!(Opt(kind!(String)))
            })))
        })])]
    );

    Ok(())
}

#[test]
fn query_with_nested_optional_all_field() -> anyhow::Result<()> {
    let query = r#"
SELECT
    id,
    xyz.* as bazza
FROM
    user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD xyz ON user TYPE option<{
    foo: option<string>,
    abc: option<string>,
    num: int
}>;
// DEFINE FIELD xyz.foo ON user TYPE option<string>;
// DEFINE FIELD xyz.abc ON user TYPE option<string>;
// DEFINE FIELD xyz.num ON user TYPE int;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            id: kind!(Record ["user"]),
            bazza: kind!(Opt(kind!({
                foo: kind!(Opt(kind!(String))),
                abc: kind!(Opt(kind!(String))),
                num: kind!(Int)
            })))
        })])]
    );

    Ok(())
}

#[test]
fn query_select_result_with_index() -> anyhow::Result<()> {
    let query = r#"
(SELECT VALUE baz FROM user)[0];
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD baz ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(return_types, vec![kind!(Opt(kind!(String)))]);

    Ok(())
}

#[test]
fn query_select_with_subquery_index() -> anyhow::Result<()> {
    let query = r#"
SELECT
    foo,
    (SELECT VALUE baz FROM user)[0] as bar
FROM
    user;
"#;
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD baz ON user TYPE string;
DEFINE FIELD foo ON user TYPE string;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            foo: kind!(String),
            bar: kind!(Opt(kind!(String)))
        })])]
    );

    Ok(())
}

// #[test]
// fn select_from_parent_field() -> anyhow::Result<()> {
//     let query = r#"
// SELECT
//     name,
//     (SELECT name FROM ONLY $parent.best_friend) as best_friend
// FROM ONLY user;
// "#;
//     let schema = r#"
// DEFINE TABLE user SCHEMAFULL;
// DEFINE FIELD name ON user TYPE string;
// DEFINE FIELD best_friend ON user TYPE record<user>;
// "#;

//     let QueryResult { return_types, .. } =
//         surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

//     assert_eq_sorted!(
//         return_types,
//         vec![Kind::Array(Box::new(Kind::Object(
//             [
//                 ("name".into(), Kind::String),
//                 (
//                     "best_friend".into(),
//                     Kind::Option(Box::new(Kind::Object(
//                         [("name".into(), Kind::String)].into()
//                     )))
//                 )
//             ]
//             .into()
//         )))]
//     );

//     Ok(())
// }
