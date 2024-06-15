use std::collections::HashSet;

use surrealdb::sql::{statements::CreateStatement, Fields, Output};

use crate::{
    step_1_parse_sql::{CodegenParameters, CodegenTables},
    QueryReturnType,
};

use super::{
    return_types::{get_fields_return_values, get_statement_fields},
    utils::get_what_table,
};

pub fn get_create_statement_return_type(
    create: &CreateStatement,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    let is_only = create.only;

    let return_type = match &create.output {
        // default return type
        Some(Output::After) | None => get_create_fields(create, schema, variables, None)?,
        Some(Output::Before | Output::Null) => QueryReturnType::Null,
        Some(Output::None) => QueryReturnType::Never,
        Some(Output::Diff) => Err(anyhow::anyhow!("Create with returned diff not supported"))?,
        Some(Output::Fields(fields)) => get_create_fields(create, schema, variables, Some(fields))?,
    };

    if is_only {
        Ok(return_type)
    } else {
        Ok(QueryReturnType::Array(Box::new(return_type)))
    }
}

// fn get_create_fields(
//     create: &CreateStatement,
//     schema: &CodegenTables,
//     variables: &CodegenParameters,
//     fields: Option<&Fields>,
// ) -> Result<QueryReturnType, anyhow::Error> {
//     let mut return_types = Vec::new();
//     let mut used_tables = HashSet::new();

//     for value in create.what.iter() {
//         let table = get_what_table(value, variables, schema)?;

//         if used_tables.contains(&table.name) {
//             continue;
//         }
//         used_tables.insert(table.name.clone());

//         match fields {
//             Some(fields) => {
//                 let mut variables = variables.clone();

//                 variables.insert(
//                     "after".into(),
//                     QueryReturnType::Object(table.fields.clone()),
//                 );
//                 variables.insert("before".into(), QueryReturnType::Null);

//                 return_types.push(get_fields_return_values(
//                     fields,
//                     &table.fields,
//                     schema,
//                     &variables,
//                 )?);
//             }
//             None => {
//                 return_types.push(QueryReturnType::Object(table.fields.clone()));
//             }
//         }
//     }

//     if return_types.len() == 1 {
//         Ok(return_types.pop().unwrap())
//     } else {
//         Ok(QueryReturnType::Either(return_types))
//     }
// }

fn get_create_fields(
    create: &CreateStatement,
    schema: &CodegenTables,
    variables: &CodegenParameters,
    fields: Option<&Fields>,
) -> Result<QueryReturnType, anyhow::Error> {
    get_statement_fields(
        &create.what,
        schema,
        variables,
        fields,
        |fields, variables| {
            variables.insert("after".into(), QueryReturnType::Object(fields.clone()));
            variables.insert("before".into(), QueryReturnType::Null);
        },
    )
}
