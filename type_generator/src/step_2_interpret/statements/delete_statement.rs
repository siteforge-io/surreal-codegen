use surrealdb::sql::{statements::DeleteStatement, Fields, Output};

use crate::{
    step_2_interpret::{get_statement_fields, schema::QueryState},
    QueryReturnType,
};

pub fn get_delete_statement_return_type(
    delete: &DeleteStatement,
    state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    let is_only = delete.only;

    let return_type = match &delete.output {
        Some(Output::After) => QueryReturnType::Null,
        Some(Output::Before) => get_delete_fields(delete, state, None)?,
        Some(Output::Null) => QueryReturnType::Null,
        Some(Output::Diff) => Err(anyhow::anyhow!("Delete with returned diff not supported"))?,
        Some(Output::Fields(fields)) => get_delete_fields(delete, state, Some(fields))?,
        Some(Output::None) => QueryReturnType::Never,
        None => QueryReturnType::Never,
        #[allow(unreachable_patterns)]
        _ => Err(anyhow::anyhow!(format!(
            "Unknown DELETE statement type: {}",
            delete
        )))?,
    };

    if is_only {
        Ok(return_type)
    } else {
        Ok(QueryReturnType::Array(Box::new(return_type)))
    }
}

fn get_delete_fields(
    delete: &DeleteStatement,
    state: &mut QueryState,
    fields: Option<&Fields>,
) -> Result<QueryReturnType, anyhow::Error> {
    get_statement_fields(&delete.what, state, fields, |fields, state| {
        state.set_local("after", QueryReturnType::Null);
        state.set_local("before", QueryReturnType::Object(fields.clone()));

        // set all fields to null because they have been deleted
        fields
            .iter_mut()
            .for_each(|(_, value)| *value = QueryReturnType::Null);
    })
}
