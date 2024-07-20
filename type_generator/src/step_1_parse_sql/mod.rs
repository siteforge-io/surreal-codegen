mod global_parameters;
mod query;
mod schema;

pub use global_parameters::*;
pub use query::*;
pub use schema::*;

// #[derive(Debug, Clone)]
// pub struct ParseState {
//     pub global: Arc<Mutex<HashMap<String, ValueType>>>,
//     pub inferred: Arc<Mutex<HashMap<String, ValueType>>>,
//     pub defined: Arc<Mutex<HashMap<String, ValueType>>>,
//     pub locals: HashMap<String, ValueType>,
// }

// impl ParseState {
//     /// Look up a parameter, moving up in the scope chain until it is found
//     pub fn get(&self, param_name: &str) -> Option<ValueType> {
//         if let Some(return_type) = self.locals.get(param_name) {
//             return Some(return_type.clone());
//         } else if let Some(return_type) = self.defined.lock().unwrap().get(param_name) {
//             return Some(return_type.clone());
//         } else if let Some(return_type) = self.inferred.lock().unwrap().get(param_name) {
//             return Some(return_type.clone());
//         } else if let Some(return_type) = self.global.lock().unwrap().get(param_name) {
//             return Some(return_type.clone());
//         }

//         None
//     }

//     pub fn has(&self, param_name: &str) -> bool {
//         self.get(param_name).is_some()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use pretty_assertions_sorted::assert_eq_sorted;
//     use surrealdb::sql::Table;

//     use super::*;

//     #[test]
//     fn parse_tables() -> anyhow::Result<()> {
//         let schema = r#"
// DEFINE TABLE user SCHEMAFULL;
// DEFINE FIELD name ON user TYPE string;
// DEFINE FIELD age ON user TYPE int;
// DEFINE FIELD bool ON user TYPE bool;
// DEFINE FIELD datetime ON user TYPE datetime;
// DEFINE FIELD duration ON user TYPE duration;
// DEFINE FIELD decimal ON user TYPE decimal;
// DEFINE FIELD xyz ON user TYPE record<xyz>;
// DEFINE FIELD arr ON user TYPE array<string>;
// DEFINE FIELD nested_obj.abc ON user TYPE string;
// DEFINE FIELD nested_obj.xyz ON user TYPE string;
// DEFINE FIELD nested_arr.*.foo ON user TYPE string;
// DEFINE FIELD nested_arr.*.bar ON user TYPE string;
// DEFINE FIELD bar.* ON user TYPE string;
// "#;

//         let tables = get_tables(&parse_sql(schema)?)?;

//         let expected_table = TableParsed {
//             name: "user".into(),
//             fields: [
//                 ("id".into(), ValueType::Record(vec!["user".into()])),
//                 ("name".into(), ValueType::String),
//                 ("age".into(), ValueType::Int),
//                 ("bool".into(), ValueType::Bool),
//                 ("datetime".into(), ValueType::Datetime),
//                 ("duration".into(), ValueType::Duration),
//                 ("decimal".into(), ValueType::Decimal),
//                 (
//                     "xyz".into(),
//                     ValueType::Record(vec![Table::from("xyz")]),
//                 ),
//                 (
//                     "arr".into(),
//                     ValueType::Array(Box::new(ValueType::String)),
//                 ),
//                 (
//                     "nested_obj".into(),
//                     ValueType::Object(HashMap::from([
//                         ("abc".into(), ValueType::String),
//                         ("xyz".into(), ValueType::String),
//                     ])),
//                 ),
//                 (
//                     "nested_arr".into(),
//                     ValueType::Array(Box::new(ValueType::Object(HashMap::from([
//                         ("bar".into(), ValueType::String),
//                         ("foo".into(), ValueType::String),
//                     ])))),
//                 ),
//                 (
//                     "bar".into(),
//                     ValueType::Array(Box::new(ValueType::String)),
//                 ),
//             ]
//             .into(),
//         };

//         assert_eq_sorted!(tables, [("user".into(), expected_table),].into());

//         Ok(())
//     }

//     #[test]
//     fn parse_views() -> anyhow::Result<()> {
//         let schema = r#"
// DEFINE TABLE user SCHEMAFULL;
// DEFINE FIELD name ON user TYPE string;
// DEFINE FIELD age ON user TYPE int;

// DEFINE TABLE user_view AS
//     SELECT
//         id as foo_id,
//         name,
//         age
//     FROM user;
// "#;

//         let tables = get_tables(&parse_sql(schema)?)?;

//         assert_eq_sorted!(
//             tables,
//             [
//                 (
//                     "user".into(),
//                     TableParsed {
//                         name: "user".into(),
//                         fields: [
//                             ("id".into(), ValueType::Record(vec!["user".into()])),
//                             ("name".into(), ValueType::String),
//                             ("age".into(), ValueType::Int),
//                         ]
//                         .into(),
//                     }
//                 ),
//                 (
//                     "user_view".into(),
//                     TableParsed {
//                         name: "user_view".into(),
//                         fields: [
//                             (
//                                 "foo_id".into(),
//                                 ValueType::Record(vec!["user".into()])
//                             ),
//                             (
//                                 "id".into(),
//                                 ValueType::Record(vec!["user_view".into()])
//                             ),
//                             ("name".into(), ValueType::String),
//                             ("age".into(), ValueType::Int),
//                         ]
//                         .into(),
//                     }
//                 )
//             ]
//             .into()
//         );

//         Ok(())
//     }
// }
