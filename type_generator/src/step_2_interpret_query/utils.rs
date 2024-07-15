use std::collections::HashMap;

use surrealdb::sql::{Param, Part, Thing, Value};

use crate::{
    step_1_parse_sql::{CodegenTable, ParseState, SchemaState},
    QueryReturnType,
};

pub fn get_what_table(
    what_value: &Value,
    state: &ParseState,
    schema: &SchemaState,
) -> Result<CodegenTable, anyhow::Error> {
    let table_name = match what_value {
        Value::Table(table) => Ok(table.0.clone()),
        Value::Param(Param(param_ident)) => {
            if let Some(QueryReturnType::Record(tables)) = state.get(param_ident.as_str()) {
                Ok(tables[0].0.clone())
            } else {
                Err(anyhow::anyhow!("Unsupported parameter: {}", param_ident))
            }
        }
        Value::Thing(Thing { tb, .. }) => Ok(tb.clone()),
        _ => Err(anyhow::anyhow!("Unsupported FROM value: {:#?}", what_value)),
    }?;

    let table = schema
        .get(&table_name)
        .ok_or_else(|| anyhow::anyhow!("Unknown table: {}", table_name))?;

    Ok(table.clone())
}

pub fn merge_into_map_recursively(
    map: &mut HashMap<String, QueryReturnType>,
    parts: &[Part],
    return_type: QueryReturnType,
) -> Result<(), anyhow::Error> {
    if parts.is_empty() {
        return Ok(());
    }

    match &parts[0] {
        Part::Field(field_name) => {
            if parts.len() == 1 {
                map.insert(field_name.0.clone(), return_type);
            } else {
                // check if the return type is a double optional, because something like xyz.abc returns option<option<string>> if xyz and abc are both optional
                if is_double_optional(&return_type) {
                    let next_map = map.entry(field_name.to_string()).or_insert_with(|| {
                        QueryReturnType::Option(Box::new(QueryReturnType::Object(HashMap::new())))
                    });

                    match next_map {
                        QueryReturnType::Option(box QueryReturnType::Object(nested_fields)) => {
                            merge_into_map_recursively(
                                nested_fields,
                                &parts[1..],
                                return_type.expect_option()?,
                            )?
                        }
                        _ => Err(anyhow::anyhow!("Unsupported return type: {:?}", next_map))?,
                    }
                } else {
                    let next_map = map
                        .entry(field_name.to_string())
                        .or_insert_with(|| QueryReturnType::Object(HashMap::new()));

                    match next_map {
                        QueryReturnType::Object(nested_fields) => {
                            merge_into_map_recursively(nested_fields, &parts[1..], return_type)?
                        }
                        _ => Err(anyhow::anyhow!("Unsupported return type: {:?}", next_map))?,
                    }
                }
            }
        }
        Part::All => {
            let array_type = QueryReturnType::Array(Box::new(return_type));
            if let Some(Part::Field(ident)) = parts.get(1) {
                map.insert(ident.to_string(), array_type);
            } else {
                map.insert("*".to_string(), array_type);
            }
        }
        _ => Err(anyhow::anyhow!(
            "Unsupported part in merge_into_map_recursively: {:?}",
            parts
        ))?,
    }

    Ok(())
}

pub fn is_double_optional(return_type: &QueryReturnType) -> bool {
    match return_type {
        QueryReturnType::Option(return_type) => match **return_type {
            QueryReturnType::Option(_) => true,
            _ => false,
        },
        _ => false,
    }
}
