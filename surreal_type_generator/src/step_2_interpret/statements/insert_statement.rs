use surrealdb::sql::statements::InsertStatement;

use crate::{step_2_interpret::schema::QueryState, ValueType};

pub fn get_insert_statement_return_type(
    insert: &InsertStatement,
    state: &mut QueryState,
) -> Result<ValueType, anyhow::Error> {
    let _ = state;
    let _ = insert;
    unimplemented!()
}
