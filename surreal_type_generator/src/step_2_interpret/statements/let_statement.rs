use surrealdb::sql::{statements::SetStatement, Kind};

use crate::{kind, step_2_interpret::QueryState};

pub fn interpret_let_statement(
    let_statement: &SetStatement,
    state: &mut QueryState,
) -> anyhow::Result<Kind> {
    state.infer(&let_statement.name, match let_statement {
        SetStatement {
            kind: Some(kind), ..
        } => kind.clone(),
        _ => anyhow::bail!("Could not infer type of LET statement\n{}\nTry using a type annotation, eg:\nLET $foo: string = ...", let_statement),
    });

    return Ok(kind!(Null));
}
