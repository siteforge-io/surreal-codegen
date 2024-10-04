use colored::Colorize;
use surrealdb::sql::{Kind, Literal};

use crate::step_3_codegen::TypeData;

#[allow(dead_code)]
pub fn type_info_to_string(type_info: &TypeData) -> String {
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

pub fn indent(str: &str) -> String {
    let mut lines = Vec::new();
    for line in str.lines() {
        lines.push(format!("    {}", line));
    }
    lines.join("\n")
}
