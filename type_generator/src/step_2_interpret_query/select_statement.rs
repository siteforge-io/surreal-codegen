use crate::{
    step_1_parse_sql::{CodegenParameters, CodegenTables},
    step_2_interpret_query::return_types::get_statement_fields,
    QueryReturnType,
};

use surrealdb::sql::statements::SelectStatement;

pub fn get_select_statement_return_type(
    select: &SelectStatement,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    if select.only {
        Ok(get_select_fields(select, schema, variables)?)
    } else {
        Ok(QueryReturnType::Array(Box::new(get_select_fields(
            select, schema, variables,
        )?)))
    }
}

fn get_select_fields(
    select: &SelectStatement,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    get_statement_fields(
        &select.what,
        schema,
        variables,
        Some(&select.expr),
        |_, _| {},
    )
}
