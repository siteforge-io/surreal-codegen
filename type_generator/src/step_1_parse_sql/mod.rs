use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, Mutex},
};

use surrealdb::sql::{
    parse,
    statements::{DefineFieldStatement, DefineStatement, DefineTableStatement},
    Cast, Param, Query, Statement, Statements, Value,
};

use crate::{kind_to_return_type, merge_fields, path_to_type, QueryReturnType};

pub type SchemaState = HashMap<String, CodegenTable>;
pub type TableReturnTypes = HashMap<String, QueryReturnType>;

#[derive(Debug, Clone)]
pub struct ParseState {
    pub global: Arc<Mutex<HashMap<String, QueryReturnType>>>,
    pub inferred: Arc<Mutex<HashMap<String, QueryReturnType>>>,
    pub defined: Arc<Mutex<HashMap<String, QueryReturnType>>>,
    pub locals: HashMap<String, QueryReturnType>,
}

impl ParseState {
    /// Look up a parameter, moving up in the scope chain until it is found
    pub fn get(&self, param_name: &str) -> Option<QueryReturnType> {
        if let Some(return_type) = self.locals.get(param_name) {
            return Some(return_type.clone());
        } else if let Some(return_type) = self.defined.lock().unwrap().get(param_name) {
            return Some(return_type.clone());
        } else if let Some(return_type) = self.inferred.lock().unwrap().get(param_name) {
            return Some(return_type.clone());
        } else if let Some(return_type) = self.global.lock().unwrap().get(param_name) {
            return Some(return_type.clone());
        }

        None
    }

    pub fn has(&self, param_name: &str) -> bool {
        self.get(param_name).is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodegenTable {
    pub name: String,
    pub fields: HashMap<String, QueryReturnType>,
}

pub fn parse_sql(sql: &str) -> Result<Query, surrealdb::error::Db> {
    parse(&sql)
}

pub fn parse_value_casts(query: &str) -> Result<HashMap<String, QueryReturnType>, anyhow::Error> {
    let mut parameter_types = HashMap::new();

    for stmt in parse_sql(query)?.into_iter() {
        match stmt {
            surrealdb::sql::Statement::Value(Value::Cast(ref cast)) => match *cast.clone() {
                Cast(kind, Value::Param(Param(param_ident))) => {
                    parameter_types.insert(param_ident.to_string(), kind_to_return_type(&kind)?);
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(parameter_types)
}

pub fn parse_query(query: &str) -> Result<(Statements, ParseState), anyhow::Error> {
    // collect and filter out all the variable castings
    let mut parameter_types = HashMap::new();
    let mut new_query = Vec::new();

    for stmt in parse_sql(query)?.into_iter() {
        match stmt {
            Statement::Value(Value::Cast(ref cast)) => match *cast.clone() {
                Cast(kind, Value::Param(Param(param_ident))) => {
                    parameter_types.insert(param_ident.to_string(), kind_to_return_type(&kind)?);
                }
                _ => new_query.push(stmt),
            },
            _ => new_query.push(stmt),
        }
    }

    Ok((
        Statements(new_query),
        ParseState {
            global: Arc::new(Mutex::new(HashMap::new())),
            inferred: Arc::new(Mutex::new(HashMap::new())),
            defined: Arc::new(Mutex::new(parameter_types)),
            locals: HashMap::new(),
        },
    ))
}

pub fn get_table_definitions(query: &Query) -> Vec<DefineTableStatement> {
    let mut tables = Vec::new();
    for stmt in query.iter() {
        match stmt {
            surrealdb::sql::Statement::Define(DefineStatement::Table(table)) => {
                tables.push(table.clone());
            }
            _ => {}
        }
    }
    tables
}

pub fn get_field_definitions(query: &Query) -> Vec<DefineFieldStatement> {
    let mut fields = Vec::new();
    for stmt in query.iter() {
        match stmt {
            surrealdb::sql::Statement::Define(DefineStatement::Field(field)) => {
                fields.push(field.clone());
            }
            _ => {}
        }
    }
    fields
}

pub fn get_tables(query: &Query) -> Result<SchemaState, anyhow::Error> {
    let mut tables = HashMap::new();

    let table_definitions = get_table_definitions(query);
    let field_definitions = get_field_definitions(query);

    for table_definition in table_definitions {
        let mut fields = HashMap::new();

        // insert the implicit id field
        fields.insert(
            "id".into(),
            QueryReturnType::Record(vec![table_definition.name.clone().into()]),
        );

        for field_definition in &field_definitions {
            if field_definition.what == table_definition.name {
                let return_type = match &field_definition.kind {
                    Some(kind) => kind_to_return_type(kind)?,
                    None => {
                        return Err(anyhow::anyhow!(
                            "Unknown field kind for {}",
                            field_definition.to_string()
                        ))
                    }
                };

                let field_type = path_to_type(&field_definition.name.0, return_type);

                // Merge this field_type into the existing fields structure
                merge_fields(&mut fields, field_type);
            }
        }

        tables.insert(
            table_definition.name.to_string(),
            CodegenTable {
                name: table_definition.name.to_string(),
                fields,
            },
        );
    }

    Ok(tables)
}

#[cfg(test)]
mod tests {
    use pretty_assertions_sorted::assert_eq_sorted;
    use surrealdb::sql::Table;

    use super::*;

    #[test]
    fn parse_tables() -> anyhow::Result<()> {
        let schema = r#"
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD age ON user TYPE int;
DEFINE FIELD bool ON user TYPE bool;
DEFINE FIELD datetime ON user TYPE datetime;
DEFINE FIELD duration ON user TYPE duration;
DEFINE FIELD decimal ON user TYPE decimal;
DEFINE FIELD xyz ON user TYPE record<xyz>;
DEFINE FIELD arr ON user TYPE array<string>;
DEFINE FIELD nested_obj.abc ON user TYPE string;
DEFINE FIELD nested_obj.xyz ON user TYPE string;
DEFINE FIELD nested_arr.*.foo ON user TYPE string;
DEFINE FIELD nested_arr.*.bar ON user TYPE string;
DEFINE FIELD bar.* ON user TYPE string;
"#;

        let tables = get_tables(&parse_sql(schema)?)?;

        let expected_table = CodegenTable {
            name: "user".into(),
            fields: [
                ("id".into(), QueryReturnType::Record(vec!["user".into()])),
                ("name".into(), QueryReturnType::String),
                ("age".into(), QueryReturnType::Int),
                ("bool".into(), QueryReturnType::Bool),
                ("datetime".into(), QueryReturnType::Datetime),
                ("duration".into(), QueryReturnType::Duration),
                ("decimal".into(), QueryReturnType::Decimal),
                (
                    "xyz".into(),
                    QueryReturnType::Record(vec![Table::from("xyz")]),
                ),
                (
                    "arr".into(),
                    QueryReturnType::Array(Box::new(QueryReturnType::String)),
                ),
                (
                    "nested_obj".into(),
                    QueryReturnType::Object(HashMap::from([
                        ("abc".into(), QueryReturnType::String),
                        ("xyz".into(), QueryReturnType::String),
                    ])),
                ),
                (
                    "nested_arr".into(),
                    QueryReturnType::Array(Box::new(QueryReturnType::Object(HashMap::from([
                        ("bar".into(), QueryReturnType::String),
                        ("foo".into(), QueryReturnType::String),
                    ])))),
                ),
                (
                    "bar".into(),
                    QueryReturnType::Array(Box::new(QueryReturnType::String)),
                ),
            ]
            .into(),
        };

        assert_eq_sorted!(tables, [("user".into(), expected_table),].into());

        Ok(())
    }
}
