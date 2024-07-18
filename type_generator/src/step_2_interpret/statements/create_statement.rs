use surrealdb::sql::{statements::CreateStatement, Fields, Output};

use crate::{
    step_2_interpret::{get_statement_fields, schema::QueryState},
    QueryReturnType,
};

pub fn get_create_statement_return_type(
    create: &CreateStatement,
    state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    let is_only = create.only;

    let return_type = match &create.output {
        // default return type
        Some(Output::After) | None => get_create_fields(create, state, None)?,
        Some(Output::Before | Output::Null) => QueryReturnType::Null,
        Some(Output::None) => QueryReturnType::Never,
        Some(Output::Diff) => Err(anyhow::anyhow!("Create with returned diff not supported"))?,
        Some(Output::Fields(fields)) => get_create_fields(create, state, Some(fields))?,
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
    state: &mut QueryState,
    fields: Option<&Fields>,
) -> Result<QueryReturnType, anyhow::Error> {
    get_statement_fields(&create.what, state, fields, |fields, state| {
        state.set_local("after", QueryReturnType::Object(fields.clone()));
        state.set_local("before", QueryReturnType::Null);
        state.set_local("this", QueryReturnType::Object(fields.clone()));
    })
}
