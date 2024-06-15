use std::collections::HashMap;

use surrealdb::sql::{Field, Fields, Ident, Idiom, Part, Value};

use crate::{
    step_1_parse_sql::{CodegenParameters, CodegenTables},
    QueryReturnType,
};

use super::{get_subquery_return_type, utils::merge_into_map_recursively};

pub fn get_fields_return_values(
    fields: &Fields,
    field_types: &HashMap<String, QueryReturnType>,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    match fields {
        // returning a single value with `VALUE`
        Fields(fields, true) => {
            Ok(
                get_field_return_type(&fields[0], &field_types, &schema, variables)?
                    .pop()
                    .unwrap()
                    .1,
            )
        }
        // returning multiple values (object map)
        Fields(fields, false) => {
            let mut map = HashMap::new();

            for field in fields {
                for (idiom, return_type) in
                    get_field_return_type(field, &field_types, &schema, variables)?
                {
                    merge_into_map_recursively(&mut map, &idiom.0, return_type)?;
                }
            }

            return Ok(QueryReturnType::Object(map));
        }
    }
}

pub fn get_field_return_type(
    field: &Field,
    field_types: &HashMap<String, QueryReturnType>,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<Vec<(Idiom, QueryReturnType)>, anyhow::Error> {
    match field {
        Field::Single {
            expr,
            alias: Some(alias),
        } => Ok(vec![(
            alias.clone(),
            get_expression_return_type(expr, field_types, schema, variables)?,
        )]),
        Field::Single { expr, alias: None } => Ok(vec![(
            expr.to_idiom(),
            get_expression_return_type(expr, field_types, schema, variables)?,
        )]),
        Field::All => {
            let mut results = vec![];
            for (field_name, field_type) in field_types {
                let idiom = Idiom(vec![Part::Field(Ident(field_name.clone()))]);
                results.push((idiom, field_type.clone().into()));
            }
            Ok(results)
        }
    }
}

pub fn get_expression_return_type(
    expr: &Value,
    field_types: &HashMap<String, QueryReturnType>,
    schema: &CodegenTables,
    variables: &CodegenParameters,
) -> Result<QueryReturnType, anyhow::Error> {
    match expr {
        Value::Idiom(idiom) => get_field_from_paths(&idiom.0, &field_types, schema),
        Value::Subquery(subquery) => get_subquery_return_type(subquery, schema, variables),
        _ => Err(anyhow::anyhow!("Unsupported expression: {}", expr)),
    }
}

pub fn get_field_from_paths(
    parts: &[Part],
    field_types: &HashMap<String, QueryReturnType>,
    schema: &CodegenTables,
) -> Result<QueryReturnType, anyhow::Error> {
    match parts.first() {
        Some(Part::Field(field_name)) => match field_types.get(field_name.as_str()) {
            Some(return_type) => match_return_type(return_type, &parts, field_types, schema),
            None => Err(anyhow::anyhow!("Field not found: {}", field_name)),
        },
        Some(_) => Err(anyhow::anyhow!("Unsupported path: {:?}", parts)),
        // We're returning an actual object
        None => Ok(QueryReturnType::Object(field_types.clone())),
    }
}

pub fn match_return_type(
    return_type: &QueryReturnType,
    parts: &[Part],
    field_types: &HashMap<String, QueryReturnType>,
    schema: &CodegenTables,
) -> Result<QueryReturnType, anyhow::Error> {
    let has_next_part = parts.len() > 1;

    match return_type {
        QueryReturnType::Object(nested_fields) => {
            get_field_from_paths(&parts[1..], nested_fields, schema)
        }
        QueryReturnType::String => Ok(QueryReturnType::String),
        QueryReturnType::Int => Ok(QueryReturnType::Int),
        QueryReturnType::Float => Ok(QueryReturnType::Float),
        QueryReturnType::Datetime => Ok(QueryReturnType::Datetime),
        QueryReturnType::Duration => Ok(QueryReturnType::Duration),
        QueryReturnType::Decimal => Ok(QueryReturnType::Decimal),
        QueryReturnType::Bool => Ok(QueryReturnType::Bool),
        QueryReturnType::Record(tables) => {
            if has_next_part {
                let mut return_types = Vec::new();
                for table in tables.iter() {
                    let return_type = match schema.get(table.as_str()) {
                        Some(new_schema) => {
                            get_field_from_paths(&parts[1..], &new_schema.fields, schema)?
                        }
                        None => return Err(anyhow::anyhow!("Unknown table: {}", table)),
                    };
                    return_types.push(return_type);
                }
                if return_types.len() == 1 {
                    Ok(return_types.pop().unwrap())
                } else {
                    Ok(QueryReturnType::Either(return_types))
                }
            } else {
                Ok(QueryReturnType::Record(tables.clone()))
            }
        }
        QueryReturnType::Option(return_type) => Ok(QueryReturnType::Option(Box::new(
            match_return_type(return_type, &parts, field_types, schema)?,
        ))),
        QueryReturnType::Array(return_type) => Ok(QueryReturnType::Array(Box::new(
            match_return_type(return_type, &parts, field_types, schema)?,
        ))),
        QueryReturnType::Null => Ok(QueryReturnType::Null),
        _ => Err(anyhow::anyhow!(
            "Unsupported return type: {:?}",
            return_type
        )),
    }
}
