use std::collections::BTreeMap;

use surrealdb::sql::{parse, Cast, Param, Statement, Value};

use crate::{kind_to_return_type, ValueType};

pub struct QueryParsed {
    pub statements: Vec<Statement>,
    pub casted_parameters: BTreeMap<String, ValueType>,
}

pub fn parse_query(query: &str) -> Result<QueryParsed, anyhow::Error> {
    // collect and filter out all the variable castings
    let mut parameter_types = BTreeMap::new();
    let mut statements = Vec::new();

    for stmt in parse(query)?.into_iter() {
        match stmt {
            Statement::Value(Value::Cast(box Cast {
                0: kind,
                1: Value::Param(Param { 0: ident, .. }),
                ..
            })) => {
                parameter_types.insert(ident.to_string(), kind_to_return_type(&kind)?);
            }
            _ => statements.push(stmt),
        }
    }

    Ok(QueryParsed {
        statements,
        casted_parameters: parameter_types,
    })
}
