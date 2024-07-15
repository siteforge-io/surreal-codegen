use crate::{
    step_1_parse_sql::{ParseState, SchemaState},
    step_2_interpret_query::return_types::get_statement_fields,
    QueryReturnType,
};

use surrealdb::sql::statements::SelectStatement;

pub fn get_select_statement_return_type(
    select: &SelectStatement,
    schema: &SchemaState,
    state: &ParseState,
) -> Result<QueryReturnType, anyhow::Error> {
    if select.only {
        Ok(get_select_fields(select, schema, state)?)
    } else {
        Ok(QueryReturnType::Array(Box::new(get_select_fields(
            select, schema, state,
        )?)))
    }
}

fn get_select_fields(
    select: &SelectStatement,
    schema: &SchemaState,
    state: &ParseState,
) -> Result<QueryReturnType, anyhow::Error> {
    get_statement_fields(
        &select.what,
        schema,
        state,
        Some(&select.expr),
        |fields, state| {
            state
                .locals
                .insert("this".into(), QueryReturnType::Object(fields.clone()));
        },
    )
}
