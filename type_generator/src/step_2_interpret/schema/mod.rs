use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use surrealdb::sql::{Block, Entry, Values};

use crate::{
    step_1_parse_sql::{parse_schema, FunctionParsed, SchemaParsed, ViewParsed},
    QueryReturnType,
};

use super::{
    get_create_statement_return_type, get_delete_statement_return_type,
    get_insert_statement_return_type, get_return_statement_return_type,
    get_select_statement_return_type, get_statement_fields, get_update_statement_return_type,
};

pub struct SchemaState {
    global_variables: HashMap<String, QueryReturnType>,
    tables: HashMap<String, InterpretedTable>,
    functions: HashMap<String, InterpretedFunction>,
    uninterpreted_views: HashMap<String, ViewParsed>,
    uninterpreted_functions: HashMap<String, FunctionParsed>,
}

pub struct QueryState {
    schema: Arc<Mutex<SchemaState>>,
    defined_variables: HashMap<String, QueryReturnType>,
    inferred_variables: HashMap<String, QueryReturnType>,
    stack_variables: Vec<HashMap<String, QueryReturnType>>,
}

impl QueryState {
    pub fn new(
        schema: Arc<Mutex<SchemaState>>,
        defined_variables: HashMap<String, QueryReturnType>,
    ) -> Self {
        Self {
            schema,
            defined_variables,
            inferred_variables: HashMap::new(),
            stack_variables: Vec::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<QueryReturnType> {
        while let Some(frame) = self.stack_variables.iter().rev().next() {
            if let Some(value) = frame.get(key) {
                return Some(value.clone());
            }
        }

        if let Some(value) = self.defined_variables.get(key) {
            return Some(value.clone());
        }

        if let Some(value) = self.inferred_variables.get(key) {
            return Some(value.clone());
        }

        if let Some(value) = self.schema.lock().unwrap().global_variables.get(key) {
            return Some(value.clone());
        }

        None
    }

    pub fn push_stack_frame(&mut self) {
        self.stack_variables.push(HashMap::new());
    }

    pub fn pop_stack_frame(&mut self) {
        self.stack_variables.pop();
    }

    pub fn set_local(&mut self, key: &str, value: QueryReturnType) {
        self.stack_variables
            .last_mut()
            .unwrap()
            .insert(key.to_string(), value);
    }

    pub fn table_opt(&mut self, name: &str) -> Result<Option<InterpretedTable>, anyhow::Error> {
        let state = self.schema.lock().unwrap();

        match state.tables.get(name) {
            Some(table) => Ok(Some(table.clone())),
            None => match state.uninterpreted_views.get(name).cloned() {
                None => Ok(None),
                Some(view) => {
                    drop(state);
                    let new_table = get_view_table(&view, self)?;
                    self.schema
                        .lock()
                        .unwrap()
                        .tables
                        .insert(name.to_string(), new_table.clone());
                    Ok(Some(new_table))
                }
            },
        }
    }

    pub fn table(&mut self, name: &str) -> Result<InterpretedTable, anyhow::Error> {
        self.table_opt(name)?
            .ok_or_else(|| anyhow::anyhow!("Unknown table: {}", name))
    }

    pub fn function_opt(
        &mut self,
        name: &str,
    ) -> Result<Option<InterpretedFunction>, anyhow::Error> {
        let state = self.schema.lock().unwrap();

        match state.functions.get(name) {
            Some(func) => Ok(Some(func.clone())),
            None => match state.uninterpreted_functions.get(name).cloned() {
                None => Ok(None),
                Some(func) => {
                    drop(state);
                    let new_func = interpret_function_parsed(func, self)?;
                    self.schema
                        .lock()
                        .unwrap()
                        .functions
                        .insert(name.to_string(), new_func.clone());
                    Ok(Some(new_func))
                }
            },
        }
    }

    pub fn function(&mut self, name: &str) -> Result<InterpretedFunction, anyhow::Error> {
        self.function_opt(name)?
            .ok_or_else(|| anyhow::anyhow!("Unknown function: {}", name))
    }

    pub fn extract_required_variables(&self) -> HashMap<String, QueryReturnType> {
        let mut variables = HashMap::new();

        for (name, value) in self.defined_variables.iter() {
            variables.insert(name.clone(), value.clone());
        }

        for (name, value) in self.inferred_variables.iter() {
            variables.insert(name.clone(), value.clone());
        }

        // should we throw an error here for any variables that were used but not defined or inferred?

        variables
    }
}

#[derive(Debug, Clone)]
pub struct InterpretedFunction {
    pub name: String,
    pub args: Vec<(String, QueryReturnType)>,
    pub return_type: QueryReturnType,
}

#[derive(Debug, Clone)]
pub struct InterpretedTable {
    pub name: String,
    pub fields: HashMap<String, QueryReturnType>,
}

pub fn interpret_schema(
    schema: &str,
    global_variables: HashMap<String, QueryReturnType>,
) -> Result<SchemaState, anyhow::Error> {
    let SchemaParsed {
        functions,
        views,
        tables,
    } = parse_schema(schema)?;

    let mut state = SchemaState {
        global_variables,
        tables: HashMap::new(),
        functions: HashMap::new(),
        uninterpreted_functions: functions,
        uninterpreted_views: views,
    };

    for table in tables.into_values() {
        state.tables.insert(
            table.name.clone(),
            InterpretedTable {
                name: table.name,
                fields: table.fields,
            },
        );
    }

    // make all functions return QueryReturnType::Never, and then set them after the views are done, and recalculae views again
    // for func in functions.values() {
    //     state.schema.functions.insert(
    //         func.name.clone(),
    //         InterpretedFunction {
    //             name: func.name.clone(),
    //             args: func.arguments.clone(),
    //             return_type: QueryReturnType::Never,
    //         },
    //     );
    // }

    // let mut view_deque = VecDeque::new();

    // for view in views.values() {
    //     view_deque.push_back(view);
    // }

    // while let Some(view) = view_deque.pop_front() {
    // state.view
    // }
    // for table in schema.tables.values() {
    //     interpret_table_parsed(&mut schmea_interpreted, table)?;
    // }

    // for func in schema.functions.into_values() {
    //     interpret_function_parsed(&mut schmea_interpreted, &func)?;
    // }

    Ok(state)
}

// fn interpret_table_parsed(state: &mut QueryState, table: &str) -> Result<(), anyhow::Error> {
//     match state.schema.tables.get(table) {
//         Some(_) => Ok(()),
//         None => match state.schema.parsed.tables.get(table) {
//             Some(TableParsed {
//                 view: Some(view), ..
//             }) => {
//                 unimplemented!()
//             }
//             Some(table) => {
//                 state.schema.tables.insert(
//                     table.name.clone(),
//                     InterpretedTable {
//                         name: table.name.clone(),
//                         fields: table.fields.clone(),
//                     },
//                 );
//                 Ok(())
//             }
//             None => anyhow::bail!("Unknown table: {}", table),
//         },
//     }
// }

fn interpret_function_parsed(
    func: FunctionParsed,
    operation_state: &mut QueryState,
) -> Result<InterpretedFunction, anyhow::Error> {
    operation_state.push_stack_frame();

    for (name, return_type) in func.arguments.clone() {
        operation_state.set_local(&name, return_type);
    }

    let func = InterpretedFunction {
        name: func.name,
        args: func.arguments,
        return_type: get_block_return_type(func.block, operation_state)?,
    };

    operation_state.pop_stack_frame();

    Ok(func)
}

fn get_block_return_type(
    block: Block,
    state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    for entry in block.0.into_iter() {
        match entry {
            Entry::Output(output) => return get_return_statement_return_type(&output, state),
            Entry::Create(create) => return get_create_statement_return_type(&create, state),
            Entry::Insert(insert) => return get_insert_statement_return_type(&insert, state),
            Entry::Delete(delete) => return get_delete_statement_return_type(&delete, state),
            Entry::Select(select) => return get_select_statement_return_type(&select, state),
            Entry::Update(update) => return get_update_statement_return_type(&update, state),
            // Entry::Upsert(upsert) => return get_upsert_statement_return_type(&upsert, state),
            _ => anyhow::bail!("Entry type: {} has not been implemented", entry),
        }
    }

    Ok(QueryReturnType::Null)
}

fn get_view_table(
    // name: &str,
    view: &ViewParsed,
    state: &mut QueryState,
) -> Result<InterpretedTable, anyhow::Error> {
    match get_view_return_type(view, state)? {
        QueryReturnType::Object(mut fields) => {
            // add the implicit id field
            if view.what.0.len() != 1 {
                return Err(anyhow::anyhow!("Expected single table in view"));
            }

            fields.insert(
                "id".into(),
                QueryReturnType::Record(vec![view.name.clone().into()]),
            );

            Ok(InterpretedTable {
                name: view.name.to_string(),
                fields,
            })
        }
        QueryReturnType::Either(..) => Err(anyhow::anyhow!(
            "Multiple tables in view are not currently supported"
        )),
        _ => Err(anyhow::anyhow!("Expected object return type"))?,
    }
}

pub fn get_view_return_type(
    view: &ViewParsed,
    state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    get_statement_fields(
        &Into::<Values>::into(&view.what),
        state,
        Some(&view.expr),
        |fields, state| {},
    )
}
