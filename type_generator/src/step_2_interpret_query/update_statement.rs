use surrealdb::sql::{statements::UpdateStatement, Fields, Output};

use crate::{
    step_1_parse_sql::{ParseState, SchemaState},
    QueryReturnType,
};

use super::return_types::get_statement_fields;

pub fn get_update_statement_return_type(
    update: &UpdateStatement,
    schema: &SchemaState,
    state: &ParseState,
) -> Result<QueryReturnType, anyhow::Error> {
    let is_only = update.only;

    let return_type = match &update.output {
        Some(Output::After) | None => get_update_fields(update, schema, state, None)?,
        Some(Output::Before) => QueryReturnType::Either(vec![
            get_update_fields(update, schema, state, None)?,
            QueryReturnType::Null,
        ]),
        Some(Output::Null) => QueryReturnType::Null,
        Some(Output::Diff) => Err(anyhow::anyhow!("Update with returned diff not supported"))?,
        Some(Output::Fields(fields)) => get_update_fields(update, schema, state, Some(fields))?,
        Some(Output::None) => QueryReturnType::Never,
    };

    if is_only {
        Ok(return_type)
    } else {
        Ok(QueryReturnType::Array(Box::new(return_type)))
    }
}

fn get_update_fields(
    update: &UpdateStatement,
    schema: &SchemaState,
    state: &ParseState,
    fields: Option<&Fields>,
) -> Result<QueryReturnType, anyhow::Error> {
    get_statement_fields(&update.what, schema, state, fields, |fields, state| {
        let mut state = state.clone();
        state
            .locals
            .insert("after".into(), QueryReturnType::Object(fields.clone()));
        state.locals.insert(
            "before".into(),
            QueryReturnType::Either(vec![
                QueryReturnType::Object(fields.clone()),
                QueryReturnType::Null,
            ]),
        );
        state
            .locals
            .insert("this".into(), QueryReturnType::Object(fields.clone()));
    })
}
