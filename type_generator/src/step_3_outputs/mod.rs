pub mod typescript;

use std::{
    collections::HashMap,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use surrealdb::sql::Query;
pub use typescript::generate_typescript_file;

use crate::{step_1_parse_sql::CodegenParameters, QueryReturnType};

pub fn query_to_return_type(
    query: &str,
    schema: &str,
) -> anyhow::Result<(CodegenParameters, Vec<QueryReturnType>, Query)> {
    let (query, parameters) = crate::step_1_parse_sql::parse_query(query)?;
    let schema_query = crate::step_1_parse_sql::parse_sql(schema)?;
    let tables = crate::step_1_parse_sql::get_tables(&schema_query)?;
    let return_types =
        crate::step_2_interpret_query::interpret_query(&query, &tables, &parameters)?;

    Ok((parameters, return_types, query))
}

pub fn read_surql_files(dir_path: &str) -> io::Result<HashMap<String, String>> {
    let path = Path::new(dir_path);
    let mut file_contents = HashMap::new();

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
