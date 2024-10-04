use crate::{
    kind,
    step_2_interpret::{return_types::get_statement_fields, schema::QueryState},
    Kind,
};

use surrealdb::sql::statements::SelectStatement;

pub fn get_select_statement_return_type(
    select: &SelectStatement,
    state: &mut QueryState,
) -> Result<Kind, anyhow::Error> {
    if select.only {
        // only will error if the select statement returns nothing
        Ok(get_select_fields(select, state)?)
    } else {
        Ok(kind!(Arr get_select_fields(select, state)?))
    }
}

fn get_select_fields(
    select: &SelectStatement,
    state: &mut QueryState,
) -> Result<Kind, anyhow::Error> {
    get_statement_fields(&select.what, state, Some(&select.expr), |fields, state| {
        state.set_local("this", kind!(Obj fields.clone()));
    })
}
