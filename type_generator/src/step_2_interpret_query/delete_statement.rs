use surrealdb::sql::{statements::DeleteStatement, Fields, Output};

use crate::{
    step_1_parse_sql::{ParseState, SchemaState},
    QueryReturnType,
};

use super::return_types::get_statement_fields;

pub fn get_delete_statement_return_type(
    delete: &DeleteStatement,
    schema: &SchemaState,
    state: &ParseState,
) -> Result<QueryReturnType, anyhow::Error> {
    let is_only = delete.only;

    let return_type = match &delete.output {
        Some(Output::After) => QueryReturnType::Null,
        Some(Output::Before) => get_delete_fields(delete, schema, state, None)?,
        Some(Output::Null) => QueryReturnType::Null,
        Some(Output::Diff) => Err(anyhow::anyhow!("Delete with returned diff not supported"))?,
        Some(Output::Fields(fields)) => get_delete_fields(delete, schema, state, Some(fields))?,
        Some(Output::None) => QueryReturnType::Never,
        None => QueryReturnType::Never,
    };

    if is_only {
        Ok(return_type)
    } else {
        Ok(QueryReturnType::Array(Box::new(return_type)))
    }
}

fn get_delete_fields(
    delete: &DeleteStatement,
    schema: &SchemaState,
    state: &ParseState,
    fields: Option<&Fields>,
) -> Result<QueryReturnType, anyhow::Error> {
    get_statement_fields(&delete.what, schema, state, fields, |fields, state| {
        state.locals.insert("after".into(), QueryReturnType::Null);
        state
            .locals
            .insert("before".into(), QueryReturnType::Object(fields.clone()));

        fields
            .iter_mut()
            .for_each(|(_, value)| *value = QueryReturnType::Null);
    })
}
