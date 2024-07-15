use pretty_assertions_sorted::assert_eq_sorted;
use std::collections::HashMap;
use type_generator::QueryReturnType;

// #[test]
// fn insert_with_single_field() -> anyhow::Result<()> {
//     let query_str = r#"
// INSERT INTO user {
//     name: "John"
// };
// "#;
//     let schema_str = r#"
// DEFINE TABLE user SCHEMAFULL;
// DEFINE FIELD name ON user TYPE string;
// "#;

//     let codegen_info = type_generator::step_3_outputs::query_to_return_type(query_str, schema_str)?;

//     assert_eq_sorted!(
//         codegen_info,
//         CodegenInformation {
//             parameters: HashMap::new(),
//             return_types: vec![QueryReturnType::Array(Box::new(QueryReturnType::Object(
//                 [
//                     ("id".into(), QueryReturnType::Record(vec!["user".into()])),
//                     ("name".into(), QueryReturnType::String),
//                 ]
//                 .into()
//             )))]
//         }
//     );

//     Ok(())
// }
