mod function;
mod object;
mod query;
mod return_types;
mod schema;
mod statements;
mod utils;

use crate::ValueType;
pub use return_types::get_statement_fields;
use return_types::get_value_return_type;
use statements::*;
use std::collections::HashMap;
use surrealdb::sql::{Statement, Subquery};

pub use schema::interpret_schema;
pub use schema::QueryState;
pub use schema::SchemaState;

pub fn interpret_query(
    statements: &Vec<Statement>,
    state: &mut QueryState,
) -> Result<Vec<ValueType>, anyhow::Error> {
    statements
        .iter()
        .map(|stmt| get_statement_return_type(stmt, state))
        .collect()
}

fn get_statement_return_type(
    stmt: &Statement,
    state: &mut QueryState,
) -> Result<ValueType, anyhow::Error> {
    match stmt {
        Statement::Select(select) => get_select_statement_return_type(select, state),
        Statement::Delete(delete) => get_delete_statement_return_type(delete, state),
        Statement::Create(create) => get_create_statement_return_type(create, state),
        Statement::Insert(insert) => get_insert_statement_return_type(insert, state),
        Statement::Update(update) => get_update_statement_return_type(update, state),
        Statement::Output(output) => get_return_statement_return_type(output, state),
        Statement::Value(value) => get_value_return_type(value, &HashMap::new(), state),
        _ => Err(anyhow::anyhow!("Unsupported statement type: `{}`", stmt)),
    }
}

fn get_subquery_return_type(
    subquery: &Subquery,
    state: &mut QueryState,
) -> Result<ValueType, anyhow::Error> {
    match subquery {
        Subquery::Select(select) => get_select_statement_return_type(select, state),
        Subquery::Delete(delete) => get_delete_statement_return_type(delete, state),
        Subquery::Create(create) => get_create_statement_return_type(create, state),
        Subquery::Insert(insert) => get_insert_statement_return_type(insert, state),
        Subquery::Update(update) => get_update_statement_return_type(update, state),
        Subquery::Value(value) => get_value_return_type(value, &HashMap::new(), state),
        _ => Err(anyhow::anyhow!("Unsupported subquery type: `{}`", subquery)),
    }
}
