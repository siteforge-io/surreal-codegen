use std::collections::HashMap;

use surrealdb::sql::statements::OutputStatement;

use crate::{
    step_2_interpret::{return_types::get_value_return_type, schema::QueryState},
    ValueType,
};

pub fn get_return_statement_return_type(
    output: &OutputStatement,
    state: &mut QueryState,
) -> Result<ValueType, anyhow::Error> {
    Ok(match output {
        OutputStatement {
            what, fetch: None, ..
        } => get_value_return_type(what, &HashMap::new(), state)?,
        OutputStatement {
            what: _,
            fetch: Some(_),
            ..
        } => unimplemented!(),
    })
}
