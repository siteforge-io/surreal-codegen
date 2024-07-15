use std::path::PathBuf;

use type_generator::step_3_outputs::{
    generate_typescript_file,
    typescript::{generate_header_typescript, generate_typescript_output},
};

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

    let files = type_generator::step_3_outputs::read_surql_files(&cli.dir)?;
    let schema = type_generator::step_3_outputs::read_file(&PathBuf::from(&cli.schema))?;
    let mut output = String::new();

    output.push_str(&generate_header_typescript());

    let mut types = Vec::new();

    for (file_name, query) in files {
        let type_data = generate_typescript_file(&file_name, &query, &schema)?;
        types.push(type_data);
    }

    output.push_str(&generate_typescript_output(&types));

    std::fs::write(cli.output, output)?;

    Ok(())
}
