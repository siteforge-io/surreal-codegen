// use pretty_assertions_sorted::assert_eq_sorted;

// #[test]
// fn can_generate_typescript_for_select_query_with_value() -> anyhow::Result<()> {
//     let query_str = r#"
// SELECT VALUE name FROM user
// "#;
//     let schema_str = r#"
// DEFINE TABLE user SCHEMAFULL;
// DEFINE FIELD name ON user TYPE string;
// "#;
//     let expected_str = r#"
// export const XQuery = "SELECT VALUE name FROM user;"
// export type XQueryResult = [string[],]
// "#
//     .trim();
//     let output =
//         type_generator::step_3_outputs::generate_typescript_file("x.surql", query_str, schema_str)?
//             .types;

//     assert_eq_sorted!(output, expected_str);

//     Ok(())
// }

// #[test]
// fn can_generate_typescript_for_select_query_with_multiple_fields() -> anyhow::Result<()> {
//     let query_str = r#"
// SELECT name, age FROM user
// "#;
//     let schema_str = r#"
// DEFINE TABLE user SCHEMAFULL;
// DEFINE FIELD name ON user TYPE string;
// DEFINE FIELD age ON user TYPE int;
// "#;
//     let expected_str = r#"
// export const XQuery = "SELECT name, age FROM user;"
// export type XQueryResult = [{age:number,name:string,}[],]
// "#
//     .trim();
//     let output =
//         type_generator::step_3_outputs::generate_typescript_file("x.surql", query_str, schema_str)?
//             .types;

//     assert_eq_sorted!(output, expected_str);

//     Ok(())
// }

// #[test]
// fn select_query_with_parameter_record() -> anyhow::Result<()> {
//     let query_str = r#"
// <record<user>> $user;

// SELECT name FROM $user
// "#;
//     let schema_str = r#"
// DEFINE TABLE user SCHEMAFULL;
// DEFINE FIELD name ON user TYPE string;
// "#;
//     let expected_str = r#"
// export const XQuery = "SELECT name FROM $user;"
// export type XQueryResult = [{name:string,}[],]
// "#
//     .trim();
//     let output =
//         type_generator::step_3_outputs::generate_typescript_file("x.surql", query_str, schema_str)?
//             .types;

//     assert_eq_sorted!(output, expected_str);

//     Ok(())
// }

// #[test]
// fn select_query_with_nested_fields_and_weird_idioms() -> anyhow::Result<()> {
//     let query_str = r#"
// SELECT
//     foo as bar,
//     baz,
//     xyz.abc
// FROM
//     user;
// "#;
//     let schema_str = r#"
// DEFINE TABLE user SCHEMAFULL;
// DEFINE FIELD foo ON user TYPE string;
// DEFINE FIELD baz ON user TYPE int;
// DEFINE FIELD xyz ON user TYPE record<xyz>;

// DEFINE TABLE xyz SCHEMAFULL;
// DEFINE FIELD abc ON xyz TYPE string;
// "#;
//     let expected_str = r#"
// export const XQuery = "SELECT foo AS bar, baz, xyz.abc FROM user;"
// export type XQueryResult = [{bar:string,baz:number,xyz:{abc:string,},}[],]
// "#
//     .trim();
//     let output =
//         type_generator::step_3_outputs::generate_typescript_file("x.surql", query_str, schema_str)?
//             .types;

//     assert_eq_sorted!(output, expected_str);

//     Ok(())
// }

// #[test]
// fn select_query_with_various_primitive_types() -> anyhow::Result<()> {
//     let query_str = r#"
// SELECT
//     name,
//     age,
//     bool,
//     datetime,
//     duration,
//     decimal
// FROM
//     user;
// "#;
//     let schema_str = r#"
// DEFINE TABLE user SCHEMAFULL;
// DEFINE FIELD name ON user TYPE string;
// DEFINE FIELD age ON user TYPE int;
// DEFINE FIELD bool ON user TYPE bool;
// DEFINE FIELD datetime ON user TYPE datetime;
// DEFINE FIELD duration ON user TYPE duration;
// DEFINE FIELD decimal ON user TYPE decimal;
// "#;
//     let expected_str = r#"
// export const XQuery = "SELECT name, age, bool, datetime, duration, decimal FROM user;"
// export type XQueryResult = [{age:number,bool:boolean,datetime:Date,decimal:Decimal,duration:Duration,name:string,}[],]
// "#.trim();
//     let output =
//         type_generator::step_3_outputs::generate_typescript_file("x.surql", query_str, schema_str)?
//             .types;

//     assert_eq_sorted!(output, expected_str);

//     Ok(())
// }

// #[test]
// fn select_query_with_only_value() -> anyhow::Result<()> {
//     let query_str = r#"
// SELECT
//     name
// FROM ONLY user;
// "#;
//     let schema_str = r#"
// DEFINE TABLE user SCHEMAFULL;
// DEFINE FIELD name ON user TYPE string;
// "#;
//     let expected_str = r#"
// export const XQuery = "SELECT name FROM ONLY user;"
// export type XQueryResult = [{name:string,},]
// "#
//     .trim();
//     let output =
//         type_generator::step_3_outputs::generate_typescript_file("x.surql", query_str, schema_str)?
//             .types;

//     assert_eq_sorted!(output, expected_str);

//     Ok(())
// }

// #[test]
// fn select_query_with_nested_array_string_field() -> anyhow::Result<()> {
//     let query_str = r#"
// SELECT
//     tags.*
// FROM
//     post;
// "#;
//     let schema_str = r#"
// DEFINE TABLE post SCHEMAFULL;
// DEFINE FIELD tags ON post TYPE array<string>;
// "#;
//     let expected_str = r#"
// export const XQuery = "SELECT tags[*] FROM post;"
// export type XQueryResult = [{tags:string[],}[],]
// "#
//     .trim();
//     let output =
//         type_generator::step_3_outputs::generate_typescript_file("x.surql", query_str, schema_str)?
//             .types;

//     assert_eq_sorted!(output, expected_str);

//     Ok(())
// }

// #[test]
// fn optional_fields() -> anyhow::Result<()> {
//     let query_str = r#"
// SELECT
//     name,
//     age,
//     bool,
//     xyz.abc
// FROM
//     user;
// "#;
//     let schema_str = r#"
// DEFINE TABLE user SCHEMAFULL;
// DEFINE FIELD name ON user TYPE option<string>;
// DEFINE FIELD age ON user TYPE option<int>;
// DEFINE FIELD bool ON user TYPE option<bool>;
// DEFINE FIELD xyz ON user TYPE option<record<xyz>>;

// DEFINE TABLE xyz SCHEMAFULL;
// DEFINE FIELD abc ON xyz TYPE option<string>;
// "#;
//     let expected_str = r#"
// export const XQuery = "SELECT name, age, bool, xyz.abc FROM user;"
// export type XQueryResult = [{age:number|null,bool:boolean|null,name:string|null,xyz:{abc:string|null,}|null,}[],]
// "#.trim();
//     let output =
//         type_generator::step_3_outputs::generate_typescript_file("x.surql", query_str, schema_str)?
//             .types;

//     assert_eq_sorted!(output, expected_str);

//     Ok(())
// }
