use surrealdb::sql::statements::InsertStatement;

use crate::{
    step_1_parse_sql::{CodegenParameters, CodegenTables},
    QueryReturnType,
};

pub fn get_insert_statement_return_type(
    insert: &InsertStatement,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    let _ = variables;
    let _ = insert;
    let _ = schema;
    unimplemented!()
}
