mod function;
mod object;
mod return_types;
mod schema;
mod statements;
mod utils;

use crate::Kind;
pub use return_types::get_statement_fields;
use return_types::get_value_return_type;
use statements::*;
use std::collections::BTreeMap;
use surrealdb::sql::{Statement, Subquery};

pub use schema::interpret_schema;
pub use schema::QueryState;
pub use schema::SchemaState;

pub fn interpret_query(
    statements: &Vec<Statement>,
    state: &mut QueryState,
) -> Result<Vec<Kind>, anyhow::Error> {
    let mut results = Vec::new();
    let mut remaining_statements = statements.clone();
    // More efficient to pop from the end
    remaining_statements.reverse();

    while let Some(stmt) = remaining_statements.pop() {
        match stmt {
            Statement::Begin(_) => {
                while let Some(stmt) = remaining_statements.pop() {
                    let mut maybe_returns = Vec::new();

                    match stmt {
                        Statement::Commit(_) => {
                            // We've hit a `COMMIT` statement, so we want to return the results
                            // as we didn't run into any `RETURN` statements
                            results.extend(maybe_returns);
                            break;
                        }
                        Statement::Begin(_) => {
                            anyhow::bail!("Unexpected `BEGIN` statement in transaction block")
                        }
                        // We ran into a `RETURN` statement, so we want to just return the result
                        stmt @ Statement::Output(_) => {
                            // We ignore whatever came before since we are returning in this block
                            match get_statement_return_type(&stmt, state)? {
                                Some(kind) => results.push(kind),
                                None => {}
                            };

                            // We want to ignore any statements after the `RETURN` statement
                            // until we hit a `COMMIT` statement
                            while let Some(stmt) = remaining_statements.pop() {
                                if matches!(stmt, Statement::Commit(_)) {
                                    break;
                                }
                            }
                            // Break out of the BEGIN block
                            break;
                        }
                        // We may return these values except if we hit a `RETURN` statement
                        _ => match get_statement_return_type(&stmt, state)? {
                            Some(kind) => maybe_returns.push(kind),
                            None => {}
                        },
                    }
                }
                continue;
            }
            Statement::Commit(_) => {
                anyhow::bail!("Unexpected `COMMIT` statement in transaction block")
            }
            stmt => match get_statement_return_type(&stmt, state)? {
                Some(kind) => results.push(kind),
                None => {}
            },
        }
    }

    Ok(results)
}

fn get_statement_return_type(
    stmt: &Statement,
    state: &mut QueryState,
) -> Result<Option<Kind>, anyhow::Error> {
    Ok(Some(match stmt {
        Statement::Select(select) => get_select_statement_return_type(select, state)?,
        Statement::Delete(delete) => get_delete_statement_return_type(delete, state)?,
        Statement::Create(create) => get_create_statement_return_type(create, state)?,
        Statement::Insert(insert) => get_insert_statement_return_type(insert, state)?,
        Statement::Update(update) => get_update_statement_return_type(update, state)?,
        Statement::Output(output) => get_return_statement_return_type(output, state)?,
        Statement::Upsert(upsert) => get_upsert_statement_return_type(upsert, state)?,
        Statement::Value(value) => get_value_return_type(value, &BTreeMap::new(), state)?,
        Statement::Set(set) => interpret_let_statement(set, state)?,
        _ => anyhow::bail!("Unsupported statement type: `{}`", stmt),
    }))
}

fn get_subquery_return_type(
    subquery: &Subquery,
    state: &mut QueryState,
) -> Result<Kind, anyhow::Error> {
    match subquery {
        Subquery::Select(select) => get_select_statement_return_type(select, state),
        Subquery::Delete(delete) => get_delete_statement_return_type(delete, state),
        Subquery::Create(create) => get_create_statement_return_type(create, state),
        Subquery::Insert(insert) => get_insert_statement_return_type(insert, state),
        Subquery::Update(update) => get_update_statement_return_type(update, state),
        Subquery::Upsert(upsert) => get_upsert_statement_return_type(upsert, state),
        Subquery::Value(value) => get_value_return_type(value, &BTreeMap::new(), state),
        _ => anyhow::bail!("Unsupported subquery type: `{}`", subquery),
    }
}
