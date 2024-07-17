use surrealdb::sql::{statements::CreateStatement, Fields, Output};

use crate::{
    step_1_parse_sql::{ParseState, SchemaState},
    QueryReturnType,
};

use super::return_types::get_statement_fields;

pub fn get_create_statement_return_type(
    create: &CreateStatement,
    schema: &SchemaState,
    state: &ParseState,
) -> Result<QueryReturnType, anyhow::Error> {
    let is_only = create.only;

    let return_type = match &create.output {
        // default return type
        Some(Output::After) | None => get_create_fields(create, schema, state, None)?,
        Some(Output::Before | Output::Null) => QueryReturnType::Null,
        Some(Output::None) => QueryReturnType::Never,
        Some(Output::Diff) => Err(anyhow::anyhow!("Create with returned diff not supported"))?,
        Some(Output::Fields(fields)) => get_create_fields(create, schema, state, Some(fields))?,
        #[allow(unreachable_patterns)]
        _ => Err(anyhow::anyhow!(format!(
            "Unknown CREATE statement type: {}",
            create
        )))?,
    };

    if is_only {
        Ok(return_type)
    } else {
        Ok(QueryReturnType::Array(Box::new(return_type)))
    }
}

fn get_create_fields(
    create: &CreateStatement,
    schema: &SchemaState,
    state: &ParseState,
    fields: Option<&Fields>,
) -> Result<QueryReturnType, anyhow::Error> {
    get_statement_fields(&create.what, schema, state, fields, |fields, state| {
        state
            .locals
            .insert("after".into(), QueryReturnType::Object(fields.clone()));
        state.locals.insert("before".into(), QueryReturnType::Null);
        state
            .locals
            .insert("this".into(), QueryReturnType::Object(fields.clone()));
    })
}
