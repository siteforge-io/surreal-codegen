use std::{collections::BTreeMap, sync::Arc};

use surrealdb::sql::{Block, Entry, Values};

use crate::{
    step_1_parse_sql::{parse_schema, FunctionParsed, SchemaParsed, ViewParsed},
    ValueType,
};

use super::{
    get_create_statement_return_type, get_delete_statement_return_type,
    get_insert_statement_return_type, get_return_statement_return_type,
    get_select_statement_return_type, get_statement_fields, get_update_statement_return_type,
};

#[derive(Debug)]
pub struct SchemaState {
    global_variables: BTreeMap<String, ValueType>,
    pub schema: SchemaParsed,
}

#[derive(Debug)]
pub struct QueryState {
    pub schema: Arc<SchemaState>,
    defined_variables: BTreeMap<String, ValueType>,
    inferred_variables: BTreeMap<String, ValueType>,
    stack_variables: Vec<BTreeMap<String, ValueType>>,
}

impl QueryState {
    pub fn new(schema: Arc<SchemaState>, defined_variables: BTreeMap<String, ValueType>) -> Self {
        Self {
            schema,
            defined_variables,
            inferred_variables: BTreeMap::new(),
            stack_variables: Vec::new(),
        }
    }

    pub fn infer(&mut self, key: &str, value: ValueType) {
        self.inferred_variables.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<ValueType> {
        let mut stack_variables = self.stack_variables.iter().rev();
        while let Some(frame) = stack_variables.next() {
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

        if let Some(value) = self.schema.global_variables.get(key) {
            return Some(value.clone());
        }

        None
    }

    pub fn push_stack_frame(&mut self) {
        self.stack_variables.push(BTreeMap::new());
    }

    pub fn pop_stack_frame(&mut self) {
        self.stack_variables.pop();
    }

    pub fn set_local(&mut self, key: &str, value: ValueType) {
        self.stack_variables
            .last_mut()
            .unwrap()
            .insert(key.to_string(), value);
    }

    pub fn table_select_fields(&mut self, name: &str) -> Result<TableFields, anyhow::Error> {
        match self.schema.schema.tables.get(name) {
            Some(table) => Ok(table.compute_select_fields()),
            None => match self.schema.schema.views.get(name).cloned() {
                Some(view) => Ok(get_view_table(&view, self)?),
                None => anyhow::bail!("Unknown table: {}", name),
            },
        }
    }

    pub fn function(&mut self, name: &str) -> Result<InterpretedFunction, anyhow::Error> {
        match self.schema.schema.functions.get(name).cloned() {
            Some(func) => Ok(interpret_function_parsed(func, self)?),
            None => anyhow::bail!("Unknown function: {}", name),
        }
    }

    pub fn extract_required_variables(&self) -> BTreeMap<String, ValueType> {
        let mut variables = BTreeMap::new();

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
    pub args: Vec<(String, ValueType)>,
    pub return_type: ValueType,
}

pub type TableFields = BTreeMap<String, ValueType>;

pub fn interpret_schema(
    schema: &str,
    global_variables: BTreeMap<String, ValueType>,
) -> Result<SchemaState, anyhow::Error> {
    Ok(SchemaState {
        global_variables,
        schema: parse_schema(schema)?,
    })
}

fn interpret_function_parsed(
    func: FunctionParsed,
    operation_state: &mut QueryState,
) -> Result<InterpretedFunction, anyhow::Error> {
    operation_state.push_stack_frame();

    for (name, return_type) in func.arguments.iter() {
        operation_state.set_local(&name, return_type.clone());
    }

    let func = InterpretedFunction {
        name: func.name,
        args: func.arguments,
        return_type: get_block_return_type(func.block, operation_state)?,
    };

    operation_state.pop_stack_frame();

    Ok(func)
}

fn get_block_return_type(block: Block, state: &mut QueryState) -> Result<ValueType, anyhow::Error> {
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

    Ok(ValueType::Null)
}

fn get_view_table(
    // name: &str,
    view: &ViewParsed,
    state: &mut QueryState,
) -> Result<TableFields, anyhow::Error> {
    match get_view_return_type(view, state)? {
        ValueType::Object(mut fields) => {
            if view.what.0.len() != 1 {
                return Err(anyhow::anyhow!("Expected single table in view"));
            }

            // add the implicit id field
            fields.insert(
                "id".into(),
                ValueType::Record(vec![view.name.clone().into()]),
            );

            Ok(fields)
        }
        ValueType::Either(..) => Err(anyhow::anyhow!(
            "Multiple tables in view are not currently supported"
        )),
        _ => Err(anyhow::anyhow!("Expected object return type"))?,
    }
}

pub fn get_view_return_type(
    view: &ViewParsed,
    state: &mut QueryState,
) -> Result<ValueType, anyhow::Error> {
    get_statement_fields(
        &Into::<Values>::into(&view.what),
        state,
        Some(&view.expr),
        |_fields, _state| {},
    )
}
