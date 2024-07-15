use surrealdb::sql::statements::InsertStatement;

use crate::{
    step_1_parse_sql::{ParseState, SchemaState},
    QueryReturnType,
};

pub fn get_insert_statement_return_type(
    insert: &InsertStatement,
    schema: &SchemaState,
    state: &ParseState,
) -> Result<QueryReturnType, anyhow::Error> {
    let _ = state;
    let _ = insert;
    let _ = schema;
    unimplemented!()
}
