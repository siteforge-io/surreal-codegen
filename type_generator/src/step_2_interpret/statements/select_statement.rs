use crate::{
    step_2_interpret::{return_types::get_statement_fields, schema::QueryState},
    ValueType,
};

use surrealdb::sql::statements::SelectStatement;

pub fn get_select_statement_return_type(
    select: &SelectStatement,
    state: &mut QueryState,
) -> Result<ValueType, anyhow::Error> {
    if select.only {
        Ok(ValueType::Option(Box::new(get_select_fields(
            select, state,
        )?)))
    } else {
        Ok(ValueType::Array(Box::new(get_select_fields(
            select, state,
        )?)))
    }
}

fn get_select_fields(
    select: &SelectStatement,
    state: &mut QueryState,
) -> Result<ValueType, anyhow::Error> {
    get_statement_fields(&select.what, state, Some(&select.expr), |fields, state| {
        state.set_local("this", ValueType::Object(fields.clone()));
    })
}
