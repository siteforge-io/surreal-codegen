use std::collections::HashMap;

use surrealdb::sql::statements::OutputStatement;

use crate::{
    step_1_parse_sql::{ParseState, SchemaState},
    QueryReturnType,
};

use super::return_types::get_value_return_type;

pub fn get_return_statement_return_type(
    output: &OutputStatement,
    schema: &SchemaState,
    state: &ParseState,
) -> Result<QueryReturnType, anyhow::Error> {
    Ok(match output {
        OutputStatement {
            what, fetch: None, ..
        } => get_value_return_type(what, &HashMap::new(), schema, state)?,
        OutputStatement {
            what: _,
            fetch: Some(_),
            ..
        } => unimplemented!(),
    })
}
