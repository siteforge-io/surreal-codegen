use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, QueryResult};

#[test]
fn test_oauth_flow() -> anyhow::Result<()> {
    let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD email ON user TYPE string;

DEFINE TABLE account SCHEMAFULL;
DEFINE FIELD user ON account TYPE record<user>;
DEFINE FIELD provider ON account TYPE string;
DEFINE FIELD provider_id ON account TYPE string;
"#;

    let query = r#"
<string> $provider;
<string> $provider_id;

BEGIN;

-- Find the existing user via the existing account if one exists
-- Otherwise, we will find or create a user with the email
LET $user = (
    SELECT VALUE user FROM account WHERE
        provider = $provider AND
        provider_id = $provider_id
)[0] || (UPSERT ONLY user MERGE $new_user RETURN VALUE id);

-- We want to upsert the new user, even if they already existed
-- So that we can update their email, if it has changed in the provider... 
UPSERT $user MERGE $new_user;

-- The user might've not had an existing account with Google so we
-- want to link their user to the account, if it doesn't already exist
UPSERT ONLY account MERGE {
    user: $user,
    provider: $provider,
    provider_id: $provider_id
};

RETURN $user;

COMMIT;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(return_types, vec![kind!(Record["user"])]);

    Ok(())
}

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
//         surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

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
//         surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

//     assert_eq_sorted!(
//         return_types,
//         vec![Kind::Object(HashMap::from([(
//             "previous".to_string(),
//             Kind::Object(HashMap::from([(
//                 "last_updated".to_string(),
//                 Kind::Datetime
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
//         surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

//     assert_eq_sorted!(
//         return_types,
//         vec![Kind::Object(HashMap::from([(
//             "previous".to_string(),
//             Kind::Object(HashMap::from([(
//                 "last_updated".to_string(),
//                 Kind::Datetime
//             ),]))
//         ),]))]
//     );

//     Ok(())
// }
