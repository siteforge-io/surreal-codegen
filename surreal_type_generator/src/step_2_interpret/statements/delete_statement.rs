use surrealdb::sql::{statements::DeleteStatement, Fields, Kind, Output};

use crate::{
    kind,
    step_2_interpret::{get_statement_fields, schema::QueryState},
};

pub fn get_delete_statement_return_type(
    delete: &DeleteStatement,
    state: &mut QueryState,
) -> Result<Kind, anyhow::Error> {
    let is_only = delete.only;

    let return_type = match &delete.output {
        Some(Output::After) => Kind::Null,
        Some(Output::Before) => get_delete_fields(delete, state, None)?,
        Some(Output::Null) => Kind::Null,
        Some(Output::Diff) => Err(anyhow::anyhow!("Delete with returned diff not supported"))?,
        Some(Output::Fields(fields)) => get_delete_fields(delete, state, Some(fields))?,
        Some(Output::None) => Kind::Null,
        None => Kind::Null,
        #[allow(unreachable_patterns)]
        _ => Err(anyhow::anyhow!(format!(
            "Unknown DELETE statement type: {}",
            delete
        )))?,
    };

    if is_only {
        Ok(return_type)
    } else {
        Ok(kind!([return_type]))
    }
}

fn get_delete_fields(
    delete: &DeleteStatement,
    state: &mut QueryState,
    fields: Option<&Fields>,
) -> Result<Kind, anyhow::Error> {
    get_statement_fields(&delete.what, state, fields, |fields, state| {
        state.set_local("after", Kind::Null);
        state.set_local("before", kind!(Obj fields.clone()));

        // set all fields to null because they have been deleted
        fields.iter_mut().for_each(|(_, value)| *value = Kind::Null);
    })
}
