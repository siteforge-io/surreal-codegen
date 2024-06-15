use std::collections::HashSet;

use surrealdb::sql::{statements::DeleteStatement, Fields, Output};

use crate::{
    step_1_parse_sql::{CodegenParameters, CodegenTables},
    QueryReturnType,
};

use super::{return_types::get_fields_return_values, utils::get_what_table};

pub fn get_delete_statement_return_type(
    delete: &DeleteStatement,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    let is_only = delete.only;

    let return_type = match &delete.output {
        Some(Output::After) => QueryReturnType::Null,
        Some(Output::Before) => get_before_delete_return_type(delete, schema, variables)?,
        Some(Output::Null) => QueryReturnType::Null,
        Some(Output::Diff) => Err(anyhow::anyhow!("Delete with returned diff not supported"))?,
        Some(Output::Fields(fields)) => {
            get_delete_with_fields_return_type(delete, schema, variables, fields)?
        }
        Some(Output::None) => QueryReturnType::Never,
        None => QueryReturnType::Never,
    };

    if is_only {
        Ok(return_type)
    } else {
        Ok(QueryReturnType::Array(Box::new(return_type)))
    }
}

fn get_before_delete_return_type(
    delete: &DeleteStatement,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    let mut return_types = Vec::new();
    let mut used_tables = HashSet::new();

    for value in delete.what.iter() {
        let table = get_what_table(value, variables, schema)?;

        if used_tables.contains(&table.name) {
            continue;
        }
        used_tables.insert(table.name.clone());

        return_types.push(QueryReturnType::Object(table.fields.clone()));
    }

    if return_types.len() == 1 {
        Ok(return_types.pop().unwrap())
    } else {
        Ok(QueryReturnType::Either(return_types))
    }
}

fn get_delete_with_fields_return_type(
    delete: &DeleteStatement,
    schema: &CodegenTables,
    variables: &CodegenParameters,
    fields: &Fields,
) -> Result<QueryReturnType, anyhow::Error> {
    let mut return_types = Vec::new();
    let mut used_tables = HashSet::new();

    for value in delete.what.iter() {
        let table = get_what_table(value, variables, schema)?;

        if used_tables.contains(&table.name) {
            continue;
        }
        used_tables.insert(table.name.clone());

        let mut variables = variables.clone();

        variables.insert("after".into(), QueryReturnType::Null);
        variables.insert(
            "before".into(),
            QueryReturnType::Object(table.fields.clone()),
        );

        let mut field_types = table.fields.clone();
        // construct a null table, as the fields are considered deleted, so everything is null
        field_types
            .iter_mut()
            .for_each(|(_, value)| *value = QueryReturnType::Null);

        return_types.push(get_fields_return_values(
            fields,
            &field_types,
            schema,
            &variables,
        )?);
    }

    if return_types.len() == 1 {
        Ok(return_types.pop().unwrap())
    } else {
        Ok(QueryReturnType::Either(return_types))
    }
}
