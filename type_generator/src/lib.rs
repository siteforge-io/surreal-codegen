#![feature(box_patterns)]

use std::collections::HashMap;

use surrealdb::sql::{Kind, Part, Table};
pub mod step_1_parse_sql;
pub mod step_2_interpret;
pub mod step_3_codegen;

pub use step_3_codegen::QueryResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValueType {
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
    Object(HashMap<String, ValueType>),
    Array(Box<ValueType>),
    Either(Vec<ValueType>),
    Record(Vec<Table>),
    Option(Box<ValueType>),
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
        Kind::Point => anyhow::bail!("Points are not yet supported"),
        Kind::Bytes => anyhow::bail!("Bytes is not yet supported"),
        Kind::Geometry(_) => anyhow::bail!("Geometry is not yet supported"),
        Kind::Set(kind, _) => ValueType::Array(Box::new(kind_to_return_type(kind)?)),
        Kind::Either(_) => anyhow::bail!("Either is not yet supported"),
        #[allow(unreachable_patterns)]
        _ => anyhow::bail!("Unknown kind: {:?}", kind),
    })
}

fn path_to_type(parts: &[Part], final_type: ValueType) -> ValueType {
    if parts.is_empty() {
        return final_type;
    }

    match &parts[0] {
        Part::Field(ident) => {
            // If this is the last part, return it as an object with the final type
            if parts.len() == 1 {
                ValueType::Object(HashMap::from([(ident.to_string(), final_type)]))
            } else {
                // Otherwise, continue building the structure
                let inner_type = path_to_type(&parts[1..], final_type);
                ValueType::Object(HashMap::from([(ident.to_string(), inner_type)]))
            }
        }
        Part::All => {
            // If we encounter '*', we need to create an array type
            if parts.len() == 1 {
                // If '*' is the last part, return an array of the final type
                ValueType::Array(Box::new(final_type))
            } else {
                // Otherwise, there are more parts to process after '*'
                // So we continue and wrap the inner type in an array
                let inner_type = path_to_type(&parts[1..], final_type);
                ValueType::Array(Box::new(inner_type))
            }
        }
        _ => unreachable!("Unhandled part type in path."),
    }
}

fn merge_fields(base: &mut HashMap<String, ValueType>, new_type: ValueType) {
    if let ValueType::Object(new_fields) = new_type {
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

fn merge_fields_deep(existing: &mut ValueType, new: ValueType) {
    match (existing, new) {
        (ValueType::Object(ref mut existing_fields), ValueType::Object(new_fields)) => {
            for (key, value) in new_fields {
                if let Some(sub_existing) = existing_fields.get_mut(&key) {
                    merge_fields_deep(sub_existing, value);
                } else {
                    existing_fields.insert(key, value);
                }
            }
        }
        (ValueType::Array(ref mut existing_element_type), ValueType::Array(new_element_type)) => {
            merge_fields_deep(existing_element_type, *new_element_type);
        }
        // DEFINE FIELD xyz ON user TYPE option<object>;
        // DEFINE FIELD xyz.foo ON user TYPE option<string>;
        //
        // FROM : Option(Any)
        // THIS : Object(HashMap { foo: Option(String) })
        // INTO : Option(Object(HashMap { foo: Option(String) }))
        (existing @ ValueType::Option(box ValueType::Any), ValueType::Object(new_fields)) => {
            *existing = ValueType::Option(Box::new(ValueType::Object(new_fields)));
        }
        // DEFINE FIELD xyz ON user TYPE option<object>;
        // DEFINE FIELD xyz.foo ON user TYPE option<string>;
        // DEFINE FIELD xyz.abc ON user TYPE option<string>;

        // FROM : Option(Object(HashMap { foo: Option(String) }))
        // THIS : Object(HashMap { abc: Option(String) }))
        // INTO : Option(Object(HashMap { foo: Option(String), abc: Option(String) }))
        (
            ValueType::Option(box ValueType::Object(existing_fields)),
            ValueType::Object(new_fields),
        ) => {
            for (key, value) in new_fields {
                if let Some(sub_existing) = existing_fields.get_mut(&key) {
                    merge_fields_deep(sub_existing, value);
                } else {
                    existing_fields.insert(key, value);
                }
            }
        }

        (existing, new) => {
            *existing = new;
        }
    }
}
