use type_generator::QueryResult;

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
