use surrealdb::sql::{statements::DeleteStatement, Fields, Output};

use crate::{
    step_1_parse_sql::{CodegenParameters, CodegenTables},
    QueryReturnType,
};

use super::return_types::get_statement_fields;

pub fn get_delete_statement_return_type(
    delete: &DeleteStatement,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    let is_only = delete.only;

    let return_type = match &delete.output {
        Some(Output::After) => QueryReturnType::Null,
        Some(Output::Before) => get_delete_fields(delete, schema, variables, None)?,
        Some(Output::Null) => QueryReturnType::Null,
        Some(Output::Diff) => Err(anyhow::anyhow!("Delete with returned diff not supported"))?,
        Some(Output::Fields(fields)) => get_delete_fields(delete, schema, variables, Some(fields))?,
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
    schema: &CodegenTables,
    variables: &CodegenParameters,
    fields: Option<&Fields>,
) -> Result<QueryReturnType, anyhow::Error> {
    get_statement_fields(
        &delete.what,
        schema,
        variables,
        fields,
        |fields, variables| {
            variables.insert("after".into(), QueryReturnType::Null);
            variables.insert("before".into(), QueryReturnType::Object(fields.clone()));

            fields
                .iter_mut()
                .for_each(|(_, value)| *value = QueryReturnType::Null);
        },
    )
}
