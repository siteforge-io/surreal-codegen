use surrealdb::sql::statements::InsertStatement;

use crate::{step_2_interpret::schema::QueryState, QueryReturnType};

pub fn get_insert_statement_return_type(
    insert: &InsertStatement,
    state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    let _ = state;
    let _ = insert;
    unimplemented!()
}
