use std::collections::HashMap;

use surrealdb::sql::{parse, statements::{DefineFieldStatement, DefineStatement, DefineTableStatement}, Cast, Kind, Param, Part, Query, Statement, Statements, Value};

pub type TablesSchema = HashMap<String, TableSchema>;
pub type FieldsSchema = HashMap<String, DefineFieldStatement>;
pub type ParameterTypes = HashMap<String, Kind>;


#[derive(Debug, Clone)]

pub struct TableSchema {
    pub table_definition: DefineTableStatement,
    pub fields: FieldsSchema,
}

pub fn parse_sql(sql: &str) -> Result<Query, surrealdb::error::Db> {
    parse(&sql)
}

pub fn parse_query(query: &str) -> Result<(Query, ParameterTypes), surrealdb::error::Db> {
    // collect and filter out all the variable castings
    let mut parameter_types = HashMap::new();
    let mut new_query = Vec::new();

    for stmt in parse_sql(query)?.into_iter() {
        match stmt {
            Statement::Value(Value::Cast(ref cast)) => match *cast.clone() {
                Cast(kind, Value::Param(Param(param_ident))) => { parameter_types.insert(param_ident.to_string(), kind); },
                _ => new_query.push(stmt),
            }
            _ => new_query.push(stmt),
        }
    }

    Ok((Query(Statements(new_query)), parameter_types))
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

pub fn get_tables(query: &Query) -> Result<TablesSchema, anyhow::Error> {
    let mut tables = HashMap::new();

    let table_definitions = get_table_definitions(query);
    let field_definitions = get_field_definitions(query);

    for table_definition in table_definitions {
        let mut fields = HashMap::new();
        for field_definition in &field_definitions {
            if field_definition.what == table_definition.name {
                match &field_definition.name.0.get(0) {
                    Some(Part::Field(ident)) => fields.insert(ident.to_string(), field_definition.clone()),
                    _ => Err(anyhow::anyhow!("Unsupported field name"))?,
                };
            }
        }

        tables.insert(table_definition.name.to_string(), TableSchema {
            table_definition,
            fields,
        });
    }

    Ok(tables)
}