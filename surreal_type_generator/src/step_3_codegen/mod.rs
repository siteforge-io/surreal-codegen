pub mod typescript;

use std::{
    collections::BTreeMap,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use surrealdb::sql::Statement;

use crate::{
    step_2_interpret::{interpret_query, QueryState, SchemaState},
    ValueType,
};

pub struct QueryResult {
    pub statements: Vec<Statement>,
    pub variables: BTreeMap<String, ValueType>,
    pub state: QueryState,
    pub return_types: Vec<ValueType>,
}

pub fn query_to_return_type(query: &str, schema: &str) -> anyhow::Result<QueryResult> {
    query_to_return_type_with_globals(query, schema, &BTreeMap::new())
}

pub fn output_query_type(query: &str, schema: Arc<SchemaState>) -> anyhow::Result<QueryResult> {
    let parsed_query = crate::step_1_parse_sql::parse_query(query)?;
    let mut query_state = QueryState::new(schema, parsed_query.casted_parameters);

    Ok(QueryResult {
        return_types: interpret_query(&parsed_query.statements, &mut query_state)?,
        statements: parsed_query.statements,
        variables: query_state.extract_required_variables(),
        state: query_state,
    })
}

pub fn query_to_return_type_with_globals(
    query: &str,
    schema: &str,
    globals: &BTreeMap<String, ValueType>,
) -> anyhow::Result<QueryResult> {
    let state = crate::step_2_interpret::interpret_schema(schema, globals.clone())?;

    output_query_type(query, Arc::new(state))
}

pub fn read_surql_files(dir_path: &str) -> io::Result<BTreeMap<String, String>> {
    let path = Path::new(dir_path);
    let mut file_contents = BTreeMap::new();

    if !path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Not a directory"));
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "surql") {
            let file_name = file_path
                .file_name()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid file name"))?
                .to_string_lossy()
                .into_owned();

            file_contents.insert(file_name, read_file(&file_path)?);
        }
    }

    Ok(file_contents)
}

pub fn read_file(file_path: &PathBuf) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
