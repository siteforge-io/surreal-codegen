// use pretty_assertions_sorted::assert_eq_sorted;
// use std::collections::HashMap;
// use surreal_type_generator::ValueType;

// #[test]
// fn insert_with_single_field() -> anyhow::Result<()> {
//     let query = r#"
// INSERT INTO user {
//     name: "John"
// };
// "#;
//     let schema = r#"
// DEFINE TABLE user SCHEMAFULL;
// DEFINE FIELD name ON user TYPE string;
// "#;

//     let codegen_info = surreal_type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

//     assert_eq_sorted!(
//         codegen_info,
//         CodegenInformation {
//             parameters: HashMap::new(),
//             return_types: vec![ValueType::Array(Box::new(ValueType::Object(
//                 [
//                     ("id".into(), ValueType::Record(vec!["user".into()])),
//                     ("name".into(), ValueType::String),
//                 ]
//                 .into()
//             )))]
//         }
//     );

//     Ok(())
// }
