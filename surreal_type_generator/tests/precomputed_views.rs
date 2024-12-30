use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{kind, QueryResult};

#[test]
fn precomputed_views() -> anyhow::Result<()> {
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
DEFINE FIELD num ON TABLE foo TYPE number;

DEFINE TABLE baz AS SELECT
    *
FROM foo;
"#;

    let query = r#"SELECT * FROM baz;"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            id: kind!(Record ["baz"]),
            num: kind!(Number)
        })])]
    );

    Ok(())
}

#[test]
fn precomputed_views_with_new_fields() -> anyhow::Result<()> {
    let schema = r#"
DEFINE TABLE foo SCHEMAFULL;
DEFINE FIELD num ON TABLE foo TYPE number;
DEFINE FIELD beep ON TABLE foo TYPE number;

DEFINE TABLE baz AS SELECT
    *,
    5 as five
FROM foo;
"#;

    let query = r#"
SELECT num, five FROM baz;
SELECT * FROM baz;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![
            kind!([kind!({
                num: kind!(Number),
                five: kind!(Number)
            })]),
            kind!([kind!({
                id: kind!(Record ["baz"]),
                num: kind!(Number),
                five: kind!(Number),
                beep: kind!(Number)
            })])
        ]
    );

    Ok(())
}

#[test]
fn roadmap_vote_counts() -> anyhow::Result<()> {
    let schema = r#"
DEFINE TABLE roadmap_vote SCHEMAFULL
    PERMISSIONS
        FOR SELECT WHERE true
        FOR CREATE WHERE user = $auth
        FOR DELETE WHERE user = $auth;

DEFINE FIELD id ON roadmap_vote TYPE string;
DEFINE FIELD user ON roadmap_vote TYPE record<user>;
DEFINE FIELD task_id ON roadmap_vote TYPE string;
DEFINE FIELD created_at ON roadmap_vote TYPE datetime VALUE time::now() READONLY;

DEFINE INDEX unique_user_task ON roadmap_vote FIELDS user, task_id UNIQUE;

DEFINE TABLE roadmap_vote_counts AS 
    SELECT 
        task_id,
        count() as total_votes 
    FROM roadmap_vote 
    GROUP BY task_id;
"#;

    let query = "SELECT * FROM roadmap_vote_counts;";

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![kind!([kind!({
            id: kind!(Record ["roadmap_vote_counts"]),
            task_id: kind!(String),
            total_votes: kind!(Number)
        })])]
    );

    Ok(())
}
