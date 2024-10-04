pub mod typescript;

use std::{
    collections::BTreeMap,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use surrealdb::sql::{Statement, Statements};

use crate::{
    step_2_interpret::{interpret_query, QueryState, SchemaState},
    Kind,
};

pub struct TypeData {
    pub schema: Arc<SchemaState>,
    pub name: String,
    pub statements: Statements,
    pub return_type: Vec<Kind>,
    pub variables: BTreeMap<String, Kind>,
}

pub fn generate_type_info(
    file_name: &str,
    query: &str,
    state: Arc<SchemaState>,
) -> Result<TypeData, anyhow::Error> {
    let result = crate::step_3_codegen::output_query_type(query, state.clone())?;
    let camel_case_file_name = filename_to_camel_case(file_name)?;

    Ok(TypeData {
        schema: state.clone(),
        name: camel_case_file_name,
        return_type: result.return_types,
        statements: {
            let mut s = Statements::default();
            s.0 = result.statements;
            s
        },
        variables: result.variables,
    })
}

fn filename_to_camel_case(filename: &str) -> Result<String, anyhow::Error> {
    let parts: Vec<&str> = filename.split('.').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!(
            "Filename must be of the form `name.extension`"
        ));
    }

    let name_part = parts[0];
    let mut camel_case_name = String::new();
    let mut new_word = true;

    for c in name_part.chars() {
        if c == '_' {
            new_word = true;
        } else if new_word {
            camel_case_name.push(c.to_uppercase().next().unwrap());
            new_word = false;
        } else {
            camel_case_name.push(c);
        }
    }

    Ok(camel_case_name)
}

pub struct QueryResult {
    pub statements: Vec<Statement>,
    pub variables: BTreeMap<String, Kind>,
    pub state: QueryState,
    pub return_types: Vec<Kind>,
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
    globals: &BTreeMap<String, Kind>,
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
