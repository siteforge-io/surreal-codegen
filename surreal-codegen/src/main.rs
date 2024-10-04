use clap::Parser;
use colored::Colorize;
use reqwest;
use semver::Version;
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};
use surreal_type_generator::{
    step_1_parse_sql, step_2_interpret,
    step_3_codegen::{self},
};

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

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

fn fetch_latest_version() -> Option<Version> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get("https://raw.githubusercontent.com/siteforge-io/surreal-codegen/main/surreal-codegen/Cargo.toml")
        .header("User-Agent", "surreal-codegen")
        .send()
        .ok()?;

    let toml_content = resp.text().ok()?;
    let parsed_toml: toml::Value = toml::from_str(&toml_content).ok()?;
    let version_str = parsed_toml.get("package")?.get("version")?.as_str()?;

    Version::parse(version_str).ok()
}

pub fn main() {
    check_latest_version();
    match interpret() {
        Ok(_) => {}
        Err(err) => {
            eprintln!(
                "{} {}",
                " ✕ Error: ".on_bright_red().bright_white().bold(),
                err.to_string()
            );
            std::process::exit(1);
        }
    }
}

fn check_latest_version() {
    if let Some(latest_version) = fetch_latest_version() {
        let current_version = Version::parse(CURRENT_VERSION).unwrap();
        if latest_version > current_version {
            println!(
                "{}",
                format!(
                    "{} A new version of {} is available: {}",
                    "➜".bright_yellow().bold(),
                    "surreal-codegen".bright_blue(),
                    latest_version.to_string().bright_yellow()
                )
                .on_red()
            );
            println!(
                "   You're currently using version {}",
                CURRENT_VERSION.bright_yellow()
            );
            println!(
                "   Update with: {}",
                "cargo install --force --git https://github.com/siteforge-io/surreal-codegen.git"
                    .bright_cyan()
            );
            println!();
        } else {
            println!(
                "{}",
                format!(
                    "{} You're using the latest version of surreal-codegen: {}",
                    "➜".bright_green().bold(),
                    CURRENT_VERSION.bright_green()
                )
            );
        }
    } else {
        println!(
            "{} Failed to fetch latest {} version from GitHub",
            "➜".red().bold(),
            "surreal-codegen".bright_cyan()
        );
    }
}

pub fn interpret() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut files = step_3_codegen::read_surql_files(&cli.dir)?;

    let globals = if let Some(globals) = files.remove("globals.surql") {
        println!(
            "{} {}",
            "➜".bright_green().bold(),
            "Parsing globals.surql".white()
        );
        step_1_parse_sql::parse_value_casts(&globals)?
    } else {
        BTreeMap::new()
    };

    let schema = step_3_codegen::read_file(&PathBuf::from(&cli.schema))?;
    println!(
        "{} {} '{}'",
        "➜".bright_green().bold(),
        "Parsing schema in".white(),
        cli.schema.bright_green()
    );
    let state = step_2_interpret::interpret_schema(&schema, globals)?;
    println!("{} {}", "➜".bright_green().bold(), "Parsed schema".white());
    let state = Arc::new(state);

    let mut types = Vec::new();

    for (file_name, query) in files {
        println!(
            "{} {} '{}'",
            "➜".bright_green().bold(),
            "Interpreting".white(),
            file_name.bright_green()
        );
        let type_info = match step_3_codegen::generate_type_info(&file_name, &query, state.clone())
        {
            Ok(type_info) => type_info,
            Err(err) => {
                eprintln!(
                    "{} {}\n{}",
                    " ✕ Error Parsing: ".on_bright_red().bright_white().bold(),
                    file_name.bright_green(),
                    err.to_string()
                );
                std::process::exit(1);
            }
        };

        types.push(type_info);
    }

    println!(
        "{} {}",
        "➜".bright_green().bold(),
        "Generating typescript output".white()
    );
    let output = step_3_codegen::typescript::generate_typescript_output(&types, &cli.header)?;

    std::fs::write(&cli.output, output)?;
    println!(
        "{} {} '{}'",
        "➜".bright_green().bold(),
        "Wrote output to".white(),
        cli.output.bright_green()
    );

    Ok(())
}
