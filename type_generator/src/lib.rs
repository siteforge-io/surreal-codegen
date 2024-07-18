#![feature(box_patterns)]

use std::collections::HashMap;

use surrealdb::sql::{Kind, Part, Table};
pub mod step_1_parse_sql;
pub mod step_2_interpret;
pub mod step_3_codegen;

pub use step_3_codegen::QueryResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum QueryReturnType {
    Any,
    Never,
    Unknown,
    Null,
    Uuid,
    String,
    Int,
    Float,
    Number,
    Datetime,
    Duration,
    Decimal,
    Bool,
    Object(HashMap<String, QueryReturnType>),
    Array(Box<QueryReturnType>),
    Either(Vec<QueryReturnType>),
    Record(Vec<Table>),
    Option(Box<QueryReturnType>),
}

impl QueryReturnType {
    pub fn expect_option(self) -> Result<QueryReturnType, anyhow::Error> {
        match self {
            QueryReturnType::Option(return_type) => Ok(*return_type),
            _ => anyhow::bail!("Expected an option type, but got: {:?}", self),
        }
    }
}

pub fn kind_to_return_type(kind: &Kind) -> Result<QueryReturnType, anyhow::Error> {
    Ok(match kind {
        Kind::Any => QueryReturnType::Any,
        Kind::Null => QueryReturnType::Null,
        Kind::String => QueryReturnType::String,
        Kind::Int => QueryReturnType::Int,
        Kind::Float => QueryReturnType::Float,
        Kind::Datetime => QueryReturnType::Datetime,
        Kind::Duration => QueryReturnType::Duration,
        Kind::Decimal => QueryReturnType::Decimal,
        Kind::Bool => QueryReturnType::Bool,
        Kind::Number => QueryReturnType::Number,
        Kind::Record(tables) => QueryReturnType::Record(tables.clone()),
        Kind::Option(kind) => QueryReturnType::Option(Box::new(kind_to_return_type(kind)?)),
        Kind::Uuid => QueryReturnType::Uuid,
        Kind::Array(kind, _) => QueryReturnType::Array(Box::new(kind_to_return_type(kind)?)),
        Kind::Object => QueryReturnType::Any,
        Kind::Point => anyhow::bail!("Points are not yet supported"),
        Kind::Bytes => anyhow::bail!("Bytes is not yet supported"),
        Kind::Geometry(_) => anyhow::bail!("Geometry is not yet supported"),
        Kind::Set(_, _) => anyhow::bail!("Sets are not yet supported"),
        Kind::Either(_) => anyhow::bail!("Either is not yet supported"),
        #[allow(unreachable_patterns)]
        _ => anyhow::bail!("Unknown kind: {:?}", kind),
    })
}

fn path_to_type(parts: &[Part], final_type: QueryReturnType) -> QueryReturnType {
    if parts.is_empty() {
        return final_type;
    }

    match &parts[0] {
        Part::Field(ident) => {
            // If this is the last part, return it as an object with the final type
            if parts.len() == 1 {
                QueryReturnType::Object(HashMap::from([(ident.to_string(), final_type)]))
            } else {
                // Otherwise, continue building the structure
                let inner_type = path_to_type(&parts[1..], final_type);
                QueryReturnType::Object(HashMap::from([(ident.to_string(), inner_type)]))
            }
        }
        Part::All => {
            // If we encounter '*', we need to create an array type
            if parts.len() == 1 {
                // If '*' is the last part, return an array of the final type
                QueryReturnType::Array(Box::new(final_type))
            } else {
                // Otherwise, there are more parts to process after '*'
                // So we continue and wrap the inner type in an array
                let inner_type = path_to_type(&parts[1..], final_type);
                QueryReturnType::Array(Box::new(inner_type))
            }
        }
        _ => unreachable!("Unhandled part type in path."),
    }
}

fn merge_fields(base: &mut HashMap<String, QueryReturnType>, new_type: QueryReturnType) {
    // dbg!(&base, &new_type);
    if let QueryReturnType::Object(new_fields) = new_type {
        for (key, value) in new_fields {
            if let Some(existing) = base.get_mut(&key) {
                merge_fields_deep(existing, value);
            } else {
                base.insert(key, value);
            }
        }
    } else {
        panic!("Top level should always be an object in these definitions.");
    }
}

fn merge_fields_deep(existing: &mut QueryReturnType, new: QueryReturnType) {
    match (existing, new) {
        (QueryReturnType::Object(ref mut existing_fields), QueryReturnType::Object(new_fields)) => {
            for (key, value) in new_fields {
                if let Some(sub_existing) = existing_fields.get_mut(&key) {
                    merge_fields_deep(sub_existing, value);
                } else {
                    existing_fields.insert(key, value);
                }
            }
        }
        (
            QueryReturnType::Array(ref mut existing_element_type),
            QueryReturnType::Array(new_element_type),
        ) => {
            merge_fields_deep(existing_element_type, *new_element_type);
        }
        (old, new) => {
            *old = new;
        }
    }
}
