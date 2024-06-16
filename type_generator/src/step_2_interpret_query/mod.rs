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
use surrealdb::sql::{Query, Statement, Subquery};
use update_statement::get_update_statement_return_type;

use crate::{
    step_1_parse_sql::{CodegenParameters, CodegenTables},
    step_2_interpret_query::select_statement::get_select_statement_return_type,
    QueryReturnType,
};

pub fn interpret_query(
    query: &Query,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<Vec<QueryReturnType>, anyhow::Error> {
    query
        .iter()
        .map(|stmt| get_statement_return_type(stmt, schema, variables))
        .collect()
}

fn get_statement_return_type(
    stmt: &Statement,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    println!("{}", stmt.to_string());
    match stmt {
        Statement::Select(select) => get_select_statement_return_type(select, schema, variables),
        Statement::Delete(delete) => get_delete_statement_return_type(delete, schema, variables),
        Statement::Create(create) => get_create_statement_return_type(create, schema, variables),
        Statement::Insert(insert) => get_insert_statement_return_type(insert, schema, variables),
        Statement::Update(update) => get_update_statement_return_type(update, schema, variables),
        _ => Err(anyhow::anyhow!("Unsupported statement type: `{}`", stmt)),
    }
}

fn get_subquery_return_type(
    subquery: &Subquery,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    match subquery {
        Subquery::Select(select) => get_select_statement_return_type(select, schema, variables),
        Subquery::Delete(delete) => get_delete_statement_return_type(delete, schema, variables),
        Subquery::Create(create) => get_create_statement_return_type(create, schema, variables),
        Subquery::Insert(insert) => get_insert_statement_return_type(insert, schema, variables),
        Subquery::Update(update) => get_update_statement_return_type(update, schema, variables),
        Subquery::Value(value) => {
            get_expression_return_type(value, &HashMap::new(), schema, variables)
        }
        _ => Err(anyhow::anyhow!("Unsupported subquery type: `{}`", subquery)),
    }
}
