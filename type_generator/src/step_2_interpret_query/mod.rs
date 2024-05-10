use std::collections::HashMap;

use surrealdb::sql::{Query, Statement, Table};

use crate::{step_1_parse_sql::{ParameterTypes, TablesSchema}, step_2_interpret_query::select_statement_interpretation::get_select_statement_return_type};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum QueryReturnType {
    String,
    Int,
    Float,
    Datetime,
    Duration,
    Decimal,
    Bool,
    Object(HashMap<String, Box<QueryReturnType>>),
    Array(Box<QueryReturnType>),
    Either(Vec<QueryReturnType>),
    Record(Vec<Table>),
    Option(Box<QueryReturnType>),
}

pub fn interpret_query(query: &Query, schema: &TablesSchema, variables: &ParameterTypes) -> Result<Vec<QueryReturnType>, anyhow::Error> {
    query.iter().map(|stmt| get_statement_return_type(stmt, schema, variables)).collect()
}

fn get_statement_return_type(
    stmt: &Statement,
    schema: &TablesSchema,
    variables: &ParameterTypes
) -> Result<QueryReturnType, anyhow::Error> {
    println!("{}", stmt.to_string());
    match stmt {
        Statement::Select(select) => get_select_statement_return_type(select, schema, variables),
        _ => Err(anyhow::anyhow!("Unsupported statement type: {:?}", stmt))
    }
}

pub mod select_statement_interpretation {
    use core::slice::SlicePattern;
    use std::collections::{HashMap, HashSet};

    use surrealdb::sql::{statements::{DefineFieldStatement, SelectStatement}, Field, Fields, Ident, Idiom, Kind, Param, Part, Table, Value, Values};
    use crate::step_1_parse_sql::{ParameterTypes, TablesSchema};
    use super::QueryReturnType;


    pub fn get_select_statement_return_type(
        select: &SelectStatement,
        schema: &TablesSchema,
        variables: &ParameterTypes
    ) -> Result<QueryReturnType, anyhow::Error> {
        match select {
            SelectStatement {
                // returning a single value
                expr: fields,
                // the table(s) to select from
                what,
                only,
                ..
            } => {
                let return_type = get_fields_return_type(fields, what, schema, variables)?;
                if *only {
                    Ok(return_type)
                } else {
                    Ok(QueryReturnType::Array(Box::new(return_type)))
                }
            },
        }
    }

    fn get_fields_return_type(
        fields: &Fields,
        what: &Values,
        schema: &TablesSchema,
        variables: &ParameterTypes
    ) -> Result<QueryReturnType, anyhow::Error> {
        let mut return_types = Vec::new();
        let mut used_tables = HashSet::new();

        for what in what.iter() {
            let table = match what {
                Value::Table(table) => table,
                Value::Param(Param(param_ident)) => match variables.get(param_ident.as_str()) {
                    Some(Kind::Record(tables)) => match tables.get(0) {
                        Some(table) => table,
                        None => return Err(anyhow::anyhow!("Unknown table type with parameter: {}", param_ident)),
                    },
                    Some(other) => return Err(anyhow::anyhow!("Unsupported parameter type: {}", other)),
                    None => return Err(anyhow::anyhow!("Unknown parameter: {}", param_ident)),
                },
                // TODO: support other types of values
                _ => return Err(anyhow::anyhow!("Unsupported FROM value: {}", what.to_string()))
            };

            // If the table is already used, skip it, since it's a duplicated type
            if used_tables.contains(table) { continue; }

            used_tables.insert(table.clone());

            match fields {
                // returning a single value with `VALUE`
                Fields(fields, true) => return_types.push(get_field_return_type(&fields[0], table, schema)?.into_iter().next().unwrap().1),
                // returning multiple values
                Fields(fields, false) => {
                    // TODO, cleanup
                    let mut temp_res: Vec<(Idiom, QueryReturnType)> = Vec::new();
                    for field in fields.iter() {
                        temp_res.extend(get_field_return_type(field, table, schema)?);
                    }

                    let mut map = HashMap::new();

                    for (idiom, return_type) in temp_res.into_iter() {
                        merge_into_map_recursively(&mut map, &idiom.0, return_type);
                    }

                    return_types.push(QueryReturnType::Object(map));
                },
            }
        }

        // If there is only one return type, return it directly.
        // Otherwise, return an `Either` type.
        if return_types.len() == 1 {
            Ok(return_types.pop().unwrap())
        } else {
            Ok(QueryReturnType::Either(return_types))
        }
    }

    fn merge_into_map_recursively(
        map: &mut HashMap<String, Box<QueryReturnType>>,
        parts: &[Part],
        return_type: QueryReturnType)
    {
        if parts.is_empty() {
            return;
        }

        let first = parts.first().unwrap();
        if let Part::Field(ident) = first {
            if parts.len() == 1 {
                // If this is the last part, insert the return type here.
                map.insert(ident.to_string(), Box::new(return_type));
            } else {
                // Otherwise, prepare or retrieve the nested map and recurse.
                let entry = map.entry(ident.to_string()).or_insert_with(|| Box::new(QueryReturnType::Object(HashMap::new())));
                if let QueryReturnType::Object(inner_map) = &mut **entry {
                    merge_into_map_recursively(inner_map, &parts[1..], return_type);
                }
            }
        }
    }

    /// Fields can contain aliases
    /// Sometimes may not even have an idiom, instead, the stringified value of the field is used as the key/identifier
    fn get_field_return_type(field: &Field, table: &Table, schema: &TablesSchema) -> Result<Vec<(Idiom, QueryReturnType)>, anyhow::Error> {
        match field {
            Field::Single { expr, alias: Some(alias) } => Ok(vec![(alias.clone(), get_expression_return_type(expr, table, schema)?)]),
            Field::Single { expr, alias: None } => Ok(vec![(expr.to_idiom(), get_expression_return_type(expr, table, schema)?)]),
            Field::All => {
                let mut return_types = Vec::new();
                // get the table fields
                let table_schema = match schema.get(&table.0) {
                    Some(schema) => schema,
                    None => return Err(anyhow::anyhow!("Unknown table: {}", table.to_string()))
                };

                for (field_ident, _) in table_schema.fields.iter() {
                    let return_type = get_table_field_kind_for_ident(field_ident, table, schema)?;

                    // construct idiom from field ident
                    let mut idiom = Vec::new();
                    idiom.push(Part::Field(Ident::from(field_ident.as_str())));

                    return_types.push((Idiom(idiom), return_type));
                }

                Ok(return_types)
            }
        }
    }

    fn get_expression_return_type(expr: &Value, table: &Table, schema: &TablesSchema) -> Result<QueryReturnType, anyhow::Error> {
        match expr {
            Value::Idiom(idiom) => get_field_idiom_return_type(&idiom.0, table, schema),
            Value::Strand(_) => Ok(QueryReturnType::String),
            _ => Err(anyhow::anyhow!("Unsupported expression: {}", expr.to_string()))
        }
    }

    fn get_field_idiom_return_type(parts: &[Part], table: &Table, schema: &TablesSchema) -> Result<QueryReturnType, anyhow::Error> {
        match parts.first() {
            Some(part) => {
                let return_type = get_table_field_kind_for_ident(part, table, schema)?;

                match &return_type {
                    QueryReturnType::Record(tables) => {
                        let mut return_types = Vec::new();
                        for table in tables.iter() {
                            return_types.push(get_field_idiom_return_type(&parts[1..], table, schema)?);
                        }
                        // if there is only one return type, return it directly instead of using a union/either type
                        match return_types.len() {
                            1 => Ok(return_types.pop().unwrap()),
                            _ => Ok(QueryReturnType::Either(return_types)),
                        }
                    },
                    QueryReturnType::Option(box QueryReturnType::Record(tables)) => {
                        let mut return_types = Vec::new();
                        for table in tables.iter() {
                            return_types.push(get_table_field_kind_for_ident(part, table, schema)?);
                        }
                        // if there is only one return type, return it directly instead of using a union/either type
                        match return_types.len() {
                            1 => Ok(return_types.pop().unwrap()),
                            _ => Ok(QueryReturnType::Option(Box::new(QueryReturnType::Either(return_types)))),
                        }
                    },
                    _ => Ok(return_type),
                }
            },
            None => return Err(anyhow::anyhow!("Should have at least one part"))
        }

        // match parts.first().unwrap() {
        //     Part::Field(ident) => {
        //         println!("{}", ident.as_str());
        //         let return_type = get_table_field_kind_for_ident(ident.as_str(), table, schema)?;
        //         // dbg!((&return_type, parts.get(1)));
        //         match &return_type {
        //             QueryReturnType::Record(tables) => match parts.get(1) {
        //                 Some(part) => load_record_field(part, tables, schema),
        //                 None => Ok(return_type),
        //             }
        //             QueryReturnType::Option(inner_type) => match &**inner_type {
        //                 QueryReturnType::Record(tables) => match parts.get(1) {
        //                     Some(part) => load_record_field(part, tables, schema),
        //                     None => Ok(return_type),
        //                 }
        //                 _ => Ok(return_type),
        //             }
        //             _ => Ok(return_type),
        //         }
        //     }
        //     part => Err(anyhow::anyhow!("Unsupported part: {:}", part.to_string()))
        // }
    }

    fn get_table_field_kind_for_ident(part: &Part, table: &Table, schema: &TablesSchema) -> Result<QueryReturnType, anyhow::Error> {
        match schema.get(&table.0) {
            Some(schema) => match schema.fields.get(part.to_string().as_str()) {
                Some(DefineFieldStatement { kind: Some( kind ), ..}) => get_field_statement_return_value(&kind),
                Some(define_field) => Err(anyhow::anyhow!("Unsupported field: {:}", define_field)),
                None => Err(anyhow::anyhow!("Unknown field: {}", part.to_string())),
            },
            None => Err(anyhow::anyhow!("Unknown table: {}", table.to_string()))
        }
    }

    fn get_field_statement_return_value(kind: &Kind) -> Result<QueryReturnType, anyhow::Error> {
        match kind {
            Kind::String => Ok(QueryReturnType::String),
            Kind::Int => Ok(QueryReturnType::Int),
            Kind::Float => Ok(QueryReturnType::Float),
            Kind::Datetime => Ok(QueryReturnType::Datetime),
            Kind::Duration => Ok(QueryReturnType::Duration),
            Kind::Decimal => Ok(QueryReturnType::Decimal),
            Kind::Bool => Ok(QueryReturnType::Bool),
            Kind::Record(tables) => Ok(QueryReturnType::Record(tables.clone())),
            Kind::Option(kind) => Ok(QueryReturnType::Option(Box::new(get_field_statement_return_value(kind)?))),
            _ => Err(anyhow::anyhow!("Unsupported field kind: {:}", kind))
        }
    }

}


pub mod utils {


}