#![feature(box_patterns)]

use std::collections::BTreeMap;

use surrealdb::sql::{Duration, Kind, Literal, Number, Table};
pub mod step_1_parse_sql;
pub mod step_2_interpret;
pub mod step_3_codegen;

pub use step_3_codegen::QueryResult;

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub enum ValueType {
    Any,                                 // yes
    Never,                               // nope, could replace with null
    Unknown,                             // nope, could replace with any or null
    Null,                                // yes
    Bool,                                // yees
    Duration,                            // yes
    Decimal,                             // yes
    Datetime,                            // yes
    String,                              // yes
    Int,                                 // yes
    Float,                               // yes
    Number,                              // yes
    Uuid,                                // yes
    Object(BTreeMap<String, ValueType>), // yes, Literal::Object
    Array(Box<ValueType>),               // yes, Literal::Array
    Either(Vec<ValueType>),              // yes
    Record(Vec<Table>),                  // yes
    Option(Box<ValueType>),              // yes

    // Literals
    StringLiteral(String), // Literal::String
    NumberLiteral(Number), // Literal::Number
    DurationLiteral(Duration), // Literal::Duration
                           // TOOD: Sets
                           // TODO: Geometries
}

impl ValueType {
    pub fn expect_option(self) -> Result<ValueType, anyhow::Error> {
        match self {
            ValueType::Option(return_type) => Ok(*return_type),
            _ => anyhow::bail!("Expected an option type, but got: {:?}", self),
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            ValueType::Option(_) => true,
            _ => false,
        }
    }
}

pub fn kind_to_return_type(kind: &Kind) -> Result<ValueType, anyhow::Error> {
    Ok(match kind {
        Kind::Any => ValueType::Any,
        Kind::Null => ValueType::Null,
        Kind::String => ValueType::String,
        Kind::Int => ValueType::Int,
        Kind::Float => ValueType::Float,
        Kind::Datetime => ValueType::Datetime,
        Kind::Duration => ValueType::Duration,
        Kind::Decimal => ValueType::Decimal,
        Kind::Bool => ValueType::Bool,
        Kind::Number => ValueType::Number,
        Kind::Record(tables) => ValueType::Record(tables.clone()),
        Kind::Option(kind) => ValueType::Option(Box::new(kind_to_return_type(kind)?)),
        Kind::Uuid => ValueType::Uuid,
        Kind::Array(kind, _) => ValueType::Array(Box::new(kind_to_return_type(kind)?)),
        Kind::Object => ValueType::Any,
        Kind::Literal(literal) => match literal {
            Literal::String(s) => ValueType::StringLiteral(s.0.clone()),
            Literal::Number(n) => ValueType::NumberLiteral(n.clone()),
            Literal::Duration(d) => ValueType::DurationLiteral(d.clone()),
            Literal::Object(obj) => {
                let mut fields = BTreeMap::new();
                for (key, value) in obj {
                    fields.insert(key.into(), kind_to_return_type(value)?);
                }
                ValueType::Object(fields)
            }
            Literal::Array(values) => {
                let mut eithers = Vec::new();
                for value in values {
                    eithers.push(kind_to_return_type(value)?);
                }
                if eithers.len() == 1 {
                    ValueType::Array(Box::new(eithers.into_iter().next().unwrap()))
                } else {
                    ValueType::Array(Box::new(ValueType::Either(eithers)))
                }
            }
            _ => anyhow::bail!("Unknown literal: {:?}", literal),
        },
        Kind::Point => anyhow::bail!("Points are not yet supported"),
        Kind::Bytes => anyhow::bail!("Bytes is not yet supported"),
        Kind::Geometry(_) => anyhow::bail!("Geometry is not yet supported"),
        Kind::Set(kind, _) => ValueType::Array(Box::new(kind_to_return_type(kind)?)),
        Kind::Either(kinds) => {
            let mut types = Vec::new();
            for kind in kinds {
                types.push(kind_to_return_type(kind)?);
            }
            ValueType::Either(types)
        }
        #[allow(unreachable_patterns)]
        _ => anyhow::bail!("Unknown kind: {:?}", kind),
    })
}
