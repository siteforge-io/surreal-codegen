use std::collections::BTreeMap;

use surrealdb::sql::{parse, Cast, Param, Value};

pub fn parse_value_casts(query: &str) -> Result<BTreeMap<String, crate::Kind>, anyhow::Error> {
    let mut parameter_types = BTreeMap::new();

    for stmt in parse(query)?.into_iter() {
        match stmt {
            surrealdb::sql::Statement::Value(Value::Cast(box Cast {
                0: kind,
                1: Value::Param(Param { 0: ident, .. }),
                ..
            })) => {
                parameter_types.insert(ident.to_string(), kind);
            }
            _ => anyhow::bail!("Only casts eg: `<int> $param;` are supported in globals.surql"),
        }
    }

    Ok(parameter_types)
}
