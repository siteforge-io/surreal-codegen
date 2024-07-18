use crate::{
    step_2_interpret::{return_types::get_statement_fields, schema::QueryState},
    QueryReturnType,
};

use surrealdb::sql::statements::SelectStatement;

pub fn get_select_statement_return_type(
    select: &SelectStatement,
    state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    if select.only {
        Ok(get_select_fields(select, state)?)
    } else {
        Ok(QueryReturnType::Array(Box::new(get_select_fields(
            select, state,
        )?)))
    }
}

fn get_select_fields(
    select: &SelectStatement,
    state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    get_statement_fields(&select.what, state, Some(&select.expr), |fields, state| {
        state.set_local("this", QueryReturnType::Object(fields.clone()));
    })
}
