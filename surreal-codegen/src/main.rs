use clap::Parser;
use colored::Colorize;
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};
use surreal_type_generator::{
    step_1_parse_sql, step_2_interpret,
    step_3_codegen::{self, typescript::TypeData},
    Kind, Literal,
};

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

pub fn main() {
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
        let type_info =
            match step_3_codegen::typescript::generate_type_info(&file_name, &query, state.clone())
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

        // println!("{}", type_info_to_string(&type_info));

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

fn type_info_to_string(type_info: &TypeData) -> String {
    let mut lines = Vec::new();

    for (i, return_type) in type_info.return_type.iter().enumerate() {
        lines.push(format!(
            "{}{}",
            format!("-- Query Result {} --\n", i).white(),
            return_type.pretty_string(),
            // indent(&return_type.pretty_string())
        ));
    }

    lines.join("\n")
}

pub trait PrettyString {
    fn pretty_string(&self) -> String;
}

impl PrettyString for Kind {
    fn pretty_string(&self) -> String {
        match self {
            Kind::Record(tables) => format!(
                "{}{}{}{}",
                "record".yellow(),
                "<".white(),
                tables
                    .iter()
                    .map(|table| table.to_string().bright_magenta().to_string())
                    .collect::<Vec<_>>()
                    .join(" | "),
                ">".white()
            ),
            Kind::Literal(Literal::Object(fields)) => {
                let mut lines = Vec::new();

                for (key, value) in fields {
                    lines.push(format!(
                        "{}{} {}",
                        key.bright_cyan(),
                        ":".white(),
                        value.pretty_string()
                    ));
                }

                format!(
                    "{}{}{}",
                    "{\n".white(),
                    indent(&lines.join(",\n")),
                    "\n}".white(),
                )
            }
            Kind::Array(kind, ..) => format!(
                "{}{}{}{}",
                "array".yellow(),
                "<".white(),
                kind.pretty_string(),
                ">".white()
            ),
            Kind::Option(kind) => format!(
                "{}{}{}{}",
                "option".yellow(),
                "<".white(),
                kind.pretty_string(),
                ">".white()
            ),
            Kind::Either(types) => types
                .iter()
                .map(|t| t.pretty_string())
                .collect::<Vec<_>>()
                .join(&" | ".white()),

            // Kind::Any => "any".to_string(),
            // Kind::Null => "null".to_string(),
            // Kind::Bool => "boolean".to_string(),
            // Kind::Duration => "duration".to_string(),
            // Kind::Decimal => "decimal".to_string(),
            // Kind::Datetime => "datetime".to_string(),
            // Kind::String => "string".to_string(),
            // Kind::Int => "int".to_string(),
            // Kind::Float => "float".to_string(),
            // Kind::Number => "number".to_string(),
            // Kind::Uuid => "uuid".to_string(),
            // Kind::Object => format!("object"),
            kind => kind.to_string().yellow().to_string(),
        }
    }
}

fn indent(str: &str) -> String {
    let mut lines = Vec::new();
    for line in str.lines() {
        lines.push(format!("    {}", line));
    }
    lines.join("\n")
}
