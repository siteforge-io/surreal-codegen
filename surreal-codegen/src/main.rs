use std::{collections::HashMap, path::PathBuf, sync::Arc};

use type_generator::step_3_codegen::typescript::{generate_type_info, generate_typescript_output};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The directory containing the Surql files
    #[clap(short, long)]
    dir: String,

    // The database schema file
    #[clap(short, long)]
    schema: String,

    /// The name of the output file
    /// default of `types.ts`
    #[clap(short, long, default_value = "./types.ts")]
    output: String,

    /// Header to add to the top of the output file
    /// If you specify this, you must import in RecordId type and a Surreal class that has a .query(query: string, variables?: Record<string, unknown>) method
    #[clap(
        long,
        default_value = "import { type RecordId, Surreal } from 'surrealdb.js'"
    )]
    header: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut files = type_generator::step_3_codegen::read_surql_files(&cli.dir)?;

    let globals = if let Some(globals) = files.remove("globals.surql") {
        type_generator::step_1_parse_sql::parse_value_casts(&globals)?
    } else {
        HashMap::new()
    };

    let schema = type_generator::step_3_codegen::read_file(&PathBuf::from(&cli.schema))?;
    let state = type_generator::step_2_interpret::interpret_schema(&schema, globals)?;
    let state = Arc::new(state);

    let mut types = Vec::new();

    for (file_name, query) in files {
        types.push(generate_type_info(&file_name, &query, state.clone())?);
    }

    let output = generate_typescript_output(&types, &cli.header)?;

    std::fs::write(cli.output, output)?;

    Ok(())
}
