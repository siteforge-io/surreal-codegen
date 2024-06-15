use std::collections::HashSet;

use surrealdb::sql::{statements::CreateStatement, Output};

use crate::{
    step_1_parse_sql::{CodegenParameters, CodegenTables},
    QueryReturnType,
};

use super::{return_types::get_fields_return_values, utils::get_what_table};

pub fn get_create_statement_return_type(
    delete: &CreateStatement,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    let is_only = delete.only;

    unimplemented!()
}
