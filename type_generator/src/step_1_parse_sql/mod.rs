mod global_parameters;
mod query;
mod schema;

pub use global_parameters::*;
pub use query::*;
pub use schema::*;

// #[derive(Debug, Clone)]
// pub struct ParseState {
//     pub global: Arc<Mutex<HashMap<String, QueryReturnType>>>,
//     pub inferred: Arc<Mutex<HashMap<String, QueryReturnType>>>,
//     pub defined: Arc<Mutex<HashMap<String, QueryReturnType>>>,
//     pub locals: HashMap<String, QueryReturnType>,
// }

// impl ParseState {
//     /// Look up a parameter, moving up in the scope chain until it is found
//     pub fn get(&self, param_name: &str) -> Option<QueryReturnType> {
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
//                 ("id".into(), QueryReturnType::Record(vec!["user".into()])),
//                 ("name".into(), QueryReturnType::String),
//                 ("age".into(), QueryReturnType::Int),
//                 ("bool".into(), QueryReturnType::Bool),
//                 ("datetime".into(), QueryReturnType::Datetime),
//                 ("duration".into(), QueryReturnType::Duration),
//                 ("decimal".into(), QueryReturnType::Decimal),
//                 (
//                     "xyz".into(),
//                     QueryReturnType::Record(vec![Table::from("xyz")]),
//                 ),
//                 (
//                     "arr".into(),
//                     QueryReturnType::Array(Box::new(QueryReturnType::String)),
//                 ),
//                 (
//                     "nested_obj".into(),
//                     QueryReturnType::Object(HashMap::from([
//                         ("abc".into(), QueryReturnType::String),
//                         ("xyz".into(), QueryReturnType::String),
//                     ])),
//                 ),
//                 (
//                     "nested_arr".into(),
//                     QueryReturnType::Array(Box::new(QueryReturnType::Object(HashMap::from([
//                         ("bar".into(), QueryReturnType::String),
//                         ("foo".into(), QueryReturnType::String),
//                     ])))),
//                 ),
//                 (
//                     "bar".into(),
//                     QueryReturnType::Array(Box::new(QueryReturnType::String)),
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
//                             ("id".into(), QueryReturnType::Record(vec!["user".into()])),
//                             ("name".into(), QueryReturnType::String),
//                             ("age".into(), QueryReturnType::Int),
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
//                                 QueryReturnType::Record(vec!["user".into()])
//                             ),
//                             (
//                                 "id".into(),
//                                 QueryReturnType::Record(vec!["user_view".into()])
//                             ),
//                             ("name".into(), QueryReturnType::String),
//                             ("age".into(), QueryReturnType::Int),
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
