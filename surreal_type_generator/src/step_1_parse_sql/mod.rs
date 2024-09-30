mod global_parameters;
mod query;
mod schema;

pub use global_parameters::*;
pub use query::*;
pub use schema::*;

// #[derive(Debug, Clone)]
// pub struct ParseState {
//     pub global: Arc<Mutex<HashMap<String, Kind>>>,
//     pub inferred: Arc<Mutex<HashMap<String, Kind>>>,
//     pub defined: Arc<Mutex<HashMap<String, Kind>>>,
//     pub locals: HashMap<String, Kind>,
// }

// impl ParseState {
//     /// Look up a parameter, moving up in the scope chain until it is found
//     pub fn get(&self, param_name: &str) -> Option<Kind> {
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
//                 ("id".into(), Kind::Record(vec!["user".into()])),
//                 ("name".into(), Kind::String),
//                 ("age".into(), Kind::Int),
//                 ("bool".into(), Kind::Bool),
//                 ("datetime".into(), Kind::Datetime),
//                 ("duration".into(), Kind::Duration),
//                 ("decimal".into(), Kind::Decimal),
//                 (
//                     "xyz".into(),
//                     Kind::Record(vec![Table::from("xyz")]),
//                 ),
//                 (
//                     "arr".into(),
//                     Kind::Array(Box::new(Kind::String)),
//                 ),
//                 (
//                     "nested_obj".into(),
//                     Kind::Object(HashMap::from([
//                         ("abc".into(), Kind::String),
//                         ("xyz".into(), Kind::String),
//                     ])),
//                 ),
//                 (
//                     "nested_arr".into(),
//                     Kind::Array(Box::new(Kind::Object(HashMap::from([
//                         ("bar".into(), Kind::String),
//                         ("foo".into(), Kind::String),
//                     ])))),
//                 ),
//                 (
//                     "bar".into(),
//                     Kind::Array(Box::new(Kind::String)),
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
//                             ("id".into(), Kind::Record(vec!["user".into()])),
//                             ("name".into(), Kind::String),
//                             ("age".into(), Kind::Int),
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
//                                 Kind::Record(vec!["user".into()])
//                             ),
//                             (
//                                 "id".into(),
//                                 Kind::Record(vec!["user_view".into()])
//                             ),
//                             ("name".into(), Kind::String),
//                             ("age".into(), Kind::Int),
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
