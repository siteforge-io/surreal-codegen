mod create_statement;
mod delete_statement;
mod insert_statement;
mod let_statement;
mod return_statement;
mod select_statement;
mod update_statement;
mod upsert_statement;

pub use create_statement::get_create_statement_return_type;
pub use delete_statement::get_delete_statement_return_type;
pub use insert_statement::get_insert_statement_return_type;
pub use let_statement::interpret_let_statement;
pub use return_statement::get_return_statement_return_type;
pub use select_statement::get_select_statement_return_type;
pub use update_statement::get_update_statement_return_type;
pub use upsert_statement::get_upsert_statement_return_type;
