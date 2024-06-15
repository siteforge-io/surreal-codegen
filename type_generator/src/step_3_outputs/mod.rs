pub mod typescript;

pub use typescript::generate_typescript;

use crate::{step_1_parse_sql::CodegenParameters, QueryReturnType};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CodegenInformation {
    pub parameters: CodegenParameters,
    pub return_types: Vec<QueryReturnType>,
}

pub fn query_to_return_type(query: &str, schema: &str) -> anyhow::Result<CodegenInformation> {
    let (query, parameters) = crate::step_1_parse_sql::parse_query(query)?;
    let schema_query = crate::step_1_parse_sql::parse_sql(schema)?;
    let tables = crate::step_1_parse_sql::get_tables(&schema_query)?;
    let return_types = crate::step_2_interpret_query::interpret_query(&query, &tables, &parameters)?;

    Ok(CodegenInformation {
        parameters,
        return_types,
    })
}