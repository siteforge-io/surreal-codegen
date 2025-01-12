use std::collections::BTreeMap;
use surrealdb::sql::{statements::SetStatement, Kind};

use crate::step_2_interpret::{return_types::get_value_return_type, QueryState};

pub fn interpret_let_statement(
    let_statement: &SetStatement,
    state: &mut QueryState,
) -> anyhow::Result<Kind> {
    let kind = match let_statement {
        SetStatement {
            kind: Some(kind), ..
        } => kind.clone(),
        SetStatement {
            kind: None, what, ..
        } => get_value_return_type(what, &BTreeMap::new(), state)?,
    };

    state.set_local(&let_statement.name, kind);

    Ok(Kind::Null)
}
