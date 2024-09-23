use std::collections::BTreeMap;

use crate::ValueType;
use surrealdb::sql::{Ident, Param, Part, Thing, Value};

use super::schema::{QueryState, TableFields};

pub fn get_what_fields(
    what_value: &Value,
    state: &mut QueryState,
) -> Result<TableFields, anyhow::Error> {
    let table_name = match what_value {
        Value::Table(table) => Ok(table.0.clone()),
        Value::Param(Param {
            0: Ident { 0: param_ident, .. },
            ..
        }) => {
            if let Some(ValueType::Record(tables)) = state.get(param_ident.as_str()) {
                Ok(tables[0].0.clone())
            } else {
                Err(anyhow::anyhow!("Unsupported parameter: {}", param_ident))
            }
        }
        // Value::Idiom(Idiom { 0: parts }) => {
        //     unimplemented!()
        // }
        Value::Thing(Thing { tb, .. }) => Ok(tb.clone()),
        _ => Err(anyhow::anyhow!("Unsupported FROM value: {:#?}", what_value)),
    }?;

    Ok(state.table_select_fields(&table_name)?)
}

pub fn merge_into_map_recursively(
    map: &mut BTreeMap<String, ValueType>,
    parts: &[Part],
    return_type: ValueType,
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
                        ValueType::Option(Box::new(ValueType::Object(BTreeMap::new())))
                    });

                    match next_map {
                        ValueType::Option(box ValueType::Object(nested_fields)) => {
                            merge_into_map_recursively(
                                nested_fields,
                                &parts[1..],
                                return_type.expect_option()?,
                            )?
                        }
                        // TODO: If we have something like SELECT *, xyz.abc FROM xyz, it will fail because it thinks `xyz` is already a record
                        // it instead needs to replace here
                        // ValueType::Option(box ValueType::Record(tables)) => {
                        //     merge_into_map_recursively(
                        //         tables[0].1.clone(),
                        //         &parts[1..],
                        //         return_type.expect_option()?,
                        //     )?
                        // }
                        _ => Err(anyhow::anyhow!(
                            "Unsupported field return type: {:?}",
                            next_map
                        ))?,
                    }
                } else {
                    let next_map = map
                        .entry(field_name.to_string())
                        .or_insert_with(|| ValueType::Object(BTreeMap::new()));

                    match next_map {
                        ValueType::Object(nested_fields) => {
                            merge_into_map_recursively(nested_fields, &parts[1..], return_type)?
                        }
                        _ => Err(anyhow::anyhow!(
                            "Unsupported field return type: {:?}",
                            next_map
                        ))?,
                    }
                }
            }
        }
        Part::All => {
            let array_type = ValueType::Array(Box::new(return_type));
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

pub fn is_double_optional(return_type: &ValueType) -> bool {
    match return_type {
        ValueType::Option(return_type) => match **return_type {
            ValueType::Option(_) => true,
            _ => false,
        },
        _ => false,
    }
}
