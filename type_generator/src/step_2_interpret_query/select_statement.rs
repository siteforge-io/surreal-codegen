use std::collections::HashSet;

use crate::{
    step_1_parse_sql::{CodegenParameters, CodegenTables},
    QueryReturnType,
};

use surrealdb::sql::{statements::SelectStatement, Fields, Values};

use super::{return_types::get_fields_return_values, utils::get_what_table};

pub fn get_select_statement_return_type(
    select: &SelectStatement,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    match select {
        SelectStatement {
            expr: fields,
            what,
            only: true,
            ..
        } => Ok(get_what_return_values(fields, what, schema, variables)?),
        SelectStatement {
            expr: fields,
            what,
            only: false,
            ..
        } => Ok(QueryReturnType::Array(Box::new(get_what_return_values(
            fields, what, schema, variables,
        )?))),
    }
}

pub fn get_what_return_values(
    fields: &Fields,
    what: &Values,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    let mut return_types = Vec::new();
    let mut used_tables = HashSet::new();

    for value in what.iter() {
        let table = get_what_table(value, variables, schema)?;

        if used_tables.contains(&table.name) {
            continue;
        }
        used_tables.insert(table.name.clone());

        let mut variables = variables.clone();

        variables.insert(
            "after".into(),
            QueryReturnType::Object(table.fields.clone()),
        );
        variables.insert(
            "before".into(),
            QueryReturnType::Object(table.fields.clone()),
        );

        return_types.push(get_fields_return_values(
            &fields,
            &table.fields,
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
