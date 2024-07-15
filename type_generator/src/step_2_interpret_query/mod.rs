mod create_statement;
mod delete_statement;
mod insert_statement;
mod return_types;
mod select_statement;
mod update_statement;
mod utils;

use std::collections::HashMap;

use create_statement::get_create_statement_return_type;
use delete_statement::get_delete_statement_return_type;
use insert_statement::get_insert_statement_return_type;
use return_types::get_expression_return_type;
use surrealdb::sql::{Statement, Statements, Subquery};
use update_statement::get_update_statement_return_type;

use crate::{
    step_1_parse_sql::{ParseState, SchemaState},
    step_2_interpret_query::select_statement::get_select_statement_return_type,
    QueryReturnType,
};

pub fn interpret_statements(
    statements: &Statements,
    state: &ParseState,
    schema: &SchemaState,
) -> Result<Vec<QueryReturnType>, anyhow::Error> {
    statements
        .iter()
        .map(|stmt| get_statement_return_type(stmt, schema, state))
        .collect()
}

fn get_statement_return_type(
    stmt: &Statement,
    schema: &SchemaState,
    state: &ParseState,
) -> Result<QueryReturnType, anyhow::Error> {
    match stmt {
        Statement::Select(select) => get_select_statement_return_type(select, schema, state),
        Statement::Delete(delete) => get_delete_statement_return_type(delete, schema, state),
        Statement::Create(create) => get_create_statement_return_type(create, schema, state),
        Statement::Insert(insert) => get_insert_statement_return_type(insert, schema, state),
        Statement::Update(update) => get_update_statement_return_type(update, schema, state),
        _ => Err(anyhow::anyhow!("Unsupported statement type: `{}`", stmt)),
    }
}

fn get_subquery_return_type(
    subquery: &Subquery,
    schema: &SchemaState,
    state: &ParseState,
) -> Result<QueryReturnType, anyhow::Error> {
    match subquery {
        Subquery::Select(select) => get_select_statement_return_type(select, schema, state),
        Subquery::Delete(delete) => get_delete_statement_return_type(delete, schema, state),
        Subquery::Create(create) => get_create_statement_return_type(create, schema, state),
        Subquery::Insert(insert) => get_insert_statement_return_type(insert, schema, state),
        Subquery::Update(update) => get_update_statement_return_type(update, schema, state),
        Subquery::Value(value) => get_expression_return_type(value, &HashMap::new(), schema, state),
        _ => Err(anyhow::anyhow!("Unsupported subquery type: `{}`", subquery)),
    }
}
