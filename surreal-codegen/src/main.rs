use std::{collections::HashMap, path::PathBuf};

use type_generator::step_3_outputs::typescript::{generate_type_info, generate_typescript_output};

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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut files = type_generator::step_3_outputs::read_surql_files(&cli.dir)?;

    let globals = if let Some(globals) = files.remove("globals.surql") {
        type_generator::step_1_parse_sql::parse_value_casts(&globals)?
    } else {
        HashMap::new()
    };
    let schema = type_generator::step_3_outputs::read_file(&PathBuf::from(&cli.schema))?;
    let mut types = Vec::new();

    for (file_name, query) in files {
        types.push(generate_type_info(&file_name, &query, &schema, &globals)?);
    }

    let output = generate_typescript_output(&types)?;

    std::fs::write(cli.output, output)?;

    Ok(())
}
