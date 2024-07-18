use std::collections::HashMap;

use surrealdb::sql::{
    parse,
    statements::{DefineFunctionStatement, DefineStatement},
    Block, Fields, Statement, Tables,
};

use crate::{kind_to_return_type, merge_fields, path_to_type, QueryReturnType};

#[derive(Debug, Clone)]
pub struct SchemaParsed {
    pub tables: HashMap<String, TableParsed>,
    pub functions: HashMap<String, FunctionParsed>,
    pub views: HashMap<String, ViewParsed>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TableParsed {
    pub name: String,
    pub fields: HashMap<String, QueryReturnType>,
}

#[derive(Debug, Clone)]
pub struct ViewParsed {
    pub name: String,
    pub expr: Fields,
    pub what: Tables,
}

#[derive(Debug, Clone)]
pub struct FunctionParsed {
    pub name: String,
    pub arguments: Vec<(String, QueryReturnType)>,
    pub block: Block,
}

pub fn parse_schema(query: &str) -> Result<SchemaParsed, anyhow::Error> {
    let statements = parse(query)?.0;

    let mut tables = HashMap::new();
    let mut views = HashMap::new();
    let mut functions = HashMap::new();

    for stmt in statements.into_iter() {
        match stmt {
            Statement::Define(DefineStatement::Table(table)) => {
                let name = table.name.to_string();
                if tables.contains_key(&name) || views.contains_key(&name) {
                    anyhow::bail!("Duplicate table name: `{}` check if it was defined twice or if you defined a field for it before defining the table", name);
                }
                match table.view {
                    Some(view) => {
                        views.insert(
                            name.clone(),
                            ViewParsed {
                                name: name.clone(),
                                expr: view.expr.clone(),
                                what: view.what.clone(),
                            },
                        );
                    }
                    None => {
                        tables.insert(
                            name.clone(),
                            TableParsed {
                                name: name.clone(),
                                fields: [
                                    // insert the implicit id field
                                    (
                                        "id".into(),
                                        QueryReturnType::Record(vec![name.clone().into()]),
                                    ),
                                ]
                                .into(),
                            },
                        );
                    }
                }
            }
            Statement::Define(DefineStatement::Field(field)) => {
                let table = tables
                    .entry(field.what.to_string())
                    .or_insert_with(TableParsed::default);

                if views.get(&field.what.to_string()).is_some() {
                    anyhow::bail!("Fields cannot be defined on views");
                };

                let return_type = match &field.kind {
                    Some(kind) => kind_to_return_type(kind)?,
                    // could return QueryReturnType::Any here
                    None => Err(anyhow::anyhow!(
                        "You must define a type for field `{}`",
                        field.to_string()
                    ))?,
                };

                let field_type = path_to_type(&field.name.0, return_type);

                println!(
                    "table fields: {:?}, merging field_type: {:?}",
                    table.fields, field_type
                );

                // Merge this field_type into the existing fields structure
                merge_fields(&mut table.fields, field_type);

                println!("table fields after merge: {:?}", table.fields);
            }
            Statement::Define(DefineStatement::Function(DefineFunctionStatement {
                name,
                args,
                block,
                ..
            })) => {
                functions.insert(
                    name.to_string(),
                    FunctionParsed {
                        name: name.to_string(),
                        arguments: args
                            .iter()
                            .map(|(ident, kind)| {
                                Ok((ident.to_string(), kind_to_return_type(kind)?))
                            })
                            .collect::<Result<Vec<(String, QueryReturnType)>, anyhow::Error>>()?,
                        block: block.clone(),
                    },
                );
            }
            // ignore other statements
            _ => {}
        }
    }

    return Ok(SchemaParsed {
        tables,
        functions,
        views,
    });
    // let mut tables = HashMap::new();

    // let field_definitions = get_field_definitions(query);
    // let mut unresolved_views = VecDeque::new();

    // for table_definition in get_table_definitions(query) {
    //     match table_definition.view {
    //         Some(_) => unresolved_views.push_back(table_definition),
    //         None => {
    //             tables.insert(
    //                 table_definition.name.to_string(),
    //                 get_normal_table(&table_definition, &field_definitions)?,
    //             );
    //         }
    //     };
    // }

    // let mut iterations = 0;
    // // this is sorta shitty and arbitrary
    // let max_iterations = unresolved_views.len() * 3;

    // // resolve views by looping through the queue of unresolved views until all views are resolved
    // 'outer: while let Some(table_def) = unresolved_views.pop_front() {
    //     iterations += 1;
    //     if iterations > max_iterations {
    //         return Err(anyhow::anyhow!(
    //             "Circular view dependency detected, or table references non-existent table {}",
    //             table_def.name
    //         ));
    //     }
    //     let view = table_def.view.as_ref().unwrap();
    //     // check if the tables this view depends on are defined, and if not, add it back to the queue
    //     for table in &view.what.0 {
    //         if !tables.contains_key(&table.0) {
    //             unresolved_views.push_back(table_def.clone());
    //             continue 'outer;
    //         }
    //     }

    //     tables.insert(
    //         table_def.name.to_string(),
    //         get_view_table(table_def.name, &view, &schema)?,
    //     );
    // }

    // Ok(tables)
}

// pub fn get_table_definitions(query: &Query) -> Vec<DefineTableStatement> {
//     let mut tables = Vec::new();
//     for stmt in query.iter() {
//         match stmt {
//             surrealdb::sql::Statement::Define(DefineStatement::Table(table)) => {
//                 tables.push(table.clone());
//             }
//             _ => {}
//         }
//     }
//     tables
// }

// pub fn get_field_definitions(query: &Query) -> Vec<DefineFieldStatement> {
//     let mut fields = Vec::new();
//     for stmt in query.iter() {
//         match stmt {
//             surrealdb::sql::Statement::Define(DefineStatement::Field(field)) => {
//                 fields.push(field.clone());
//             }
//             _ => {}
//         }
//     }
//     fields
// }
