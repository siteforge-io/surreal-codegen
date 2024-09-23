// use std::collections::HashMap;

// use pretty_assertions_sorted::assert_eq_sorted;
// use type_generator::{QueryResult, ValueType};

// #[test]
// fn test_complex_query() -> anyhow::Result<()> {
//     let schema = r#"
// DEFINE TABLE statement SCHEMAFULL;
// DEFINE FIELD content ON TABLE statement TYPE string;

// DEFINE TABLE statement_vote SCHEMAFULL;
// DEFINE FIELD rating ON TABLE statement_vote TYPE int ASSERT $value >= 0 AND $value <= 5;
// DEFINE FIELD statement ON TABLE statement_vote TYPE record<statement> READONLY;

// DEFINE TABLE statement_rating AS
//     SELECT
//         math::mean(rating) AS rating_avg,
//         count() as total_votes,
//         statement
//     FROM statement_vote
//     GROUP BY statement;
// "#;

//     let query = r#"
// <record<statement>> $statement;

// RETURN {
//     current: (
//         SELECT
//             (
//                 SELECT rating_avg, total_votes
//                 FROM ONLY statement_rating:[$parent]
//             ) as vote_data
//         FROM ONLY $statement
//     )
// }
// "#;

//     let QueryResult { return_types, .. } =
//         type_generator::step_3_codegen::query_to_return_type(query, schema)?;

//     Ok(())
// }

// #[test]
// fn aggregate_nested_field() -> anyhow::Result<()> {
//     let schema = r#"
// DEFINE TABLE task SCHEMAFULL;
// DEFINE FIELD last_updated ON TABLE task TYPE datetime;
// DEFINE FIELD task_id ON TABLE task TYPE string;

// DEFINE TABLE task_view AS
//     SELECT
//         *,
//         (
//             SELECT VALUE id
//             FROM ONLY task
//             WHERE
//                 last_updated < $parent.last_updated
//                 AND task_id = $parent.task_id
//             LIMIT 1
//         )[0] AS previous
//     FROM task;
// "#;

//     let query = r#"
// SELECT
//     *,
//     previous.last_updated
// FROM task_view
// "#;

//     let QueryResult { return_types, .. } =
//         type_generator::step_3_codegen::query_to_return_type(query, schema)?;

//     assert_eq_sorted!(
//         return_types,
//         vec![ValueType::Object(HashMap::from([(
//             "previous".to_string(),
//             ValueType::Object(HashMap::from([(
//                 "last_updated".to_string(),
//                 ValueType::Datetime
//             ),]))
//         ),]))]
//     );

//     Ok(())
// }

// #[test]
// fn aggregate_nested_field_array() -> anyhow::Result<()> {
//     let schema = r#"
// DEFINE TABLE task SCHEMAFULL;
// DEFINE FIELD last_updated ON TABLE task TYPE datetime;
// DEFINE FIELD task_id ON TABLE task TYPE string;

// DEFINE TABLE task_view AS
//     SELECT
//         *,
//         (
//             SELECT VALUE id
//             FROM ONLY task
//             WHERE
//                 last_updated < $parent.last_updated
//                 AND task_id = $parent.task_id
//             LIMIT 1
//         ) AS previous
//     FROM task;
// "#;

//     let query = r#"
// SELECT
//     *,
//     previous.last_updated
// FROM task_view
// "#;

//     let QueryResult { return_types, .. } =
//         type_generator::step_3_codegen::query_to_return_type(query, schema)?;

//     assert_eq_sorted!(
//         return_types,
//         vec![ValueType::Object(HashMap::from([(
//             "previous".to_string(),
//             ValueType::Object(HashMap::from([(
//                 "last_updated".to_string(),
//                 ValueType::Datetime
//             ),]))
//         ),]))]
//     );

//     Ok(())
// }
