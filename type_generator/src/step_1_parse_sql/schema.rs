use std::collections::HashMap;

use surrealdb::sql::{
    parse,
    statements::{
        DefineFieldStatement, DefineFunctionStatement, DefineStatement, DefineTableStatement,
        IfelseStatement, ThrowStatement,
    },
    Block, Entry, Expression, Fields, Function, Idiom, Param, Part, Query, Statement, Tables,
    Value,
};

use crate::{kind_to_return_type, ValueType};

#[derive(Debug, PartialEq, Eq)]
pub struct SchemaParsed {
    pub tables: HashMap<String, TableParsed>,
    pub functions: HashMap<String, FunctionParsed>,
    pub views: HashMap<String, ViewParsed>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewParsed {
    pub name: String,
    pub expr: Fields,
    pub what: Tables,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionParsed {
    pub name: String,
    pub arguments: Vec<(String, ValueType)>,
    pub block: Block,
}

#[derive(Debug, PartialEq, Eq)]

pub enum FieldType {
    Simple,
    NestedObject(HashMap<String, FieldParsed>),
    NestedArray(Box<FieldParsed>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct TableParsed {
    pub name: String,
    pub fields: HashMap<String, FieldParsed>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FieldParsed {
    pub name: String,
    pub is_optional: bool,
    pub return_type: ValueType,
    pub has_default: bool,
    pub has_override_value: bool,
    pub flexible: bool,
    pub readonly: bool,
    pub field_type: FieldType,
}

impl FieldParsed {
    pub fn compute_create_type(&self) -> ValueType {
        match &self.field_type {
            FieldType::Simple => match self.is_optional || self.has_default {
                true => ValueType::Option(Box::new(self.return_type.clone())),
                false => self.return_type.clone(),
            },
            FieldType::NestedObject(obj) => {
                let mut fields = HashMap::new();
                for (key, value) in obj {
                    if !value.has_override_value {
                        fields.insert(key.clone(), value.compute_create_type());
                    }
                }

                let fields = ValueType::Object(fields);

                match self.is_optional || self.has_default {
                    true => ValueType::Option(Box::new(fields)),
                    false => fields,
                }
            }
            _ => todo!(),
        }
    }

    pub fn compute_select_type(&self) -> ValueType {
        match &self.field_type {
            FieldType::Simple => match self.is_optional {
                true => ValueType::Option(Box::new(self.return_type.clone())),
                false => self.return_type.clone(),
            },
            FieldType::NestedObject(obj) => {
                let mut fields = HashMap::new();
                for (key, value) in obj {
                    fields.insert(key.clone(), value.compute_select_type());
                }

                let fields = ValueType::Object(fields);

                match self.is_optional {
                    true => ValueType::Option(Box::new(fields)),
                    false => fields,
                }
            }
            FieldType::NestedArray(inner_type) => {
                let inner_type = inner_type.compute_select_type();
                match self.is_optional {
                    true => ValueType::Option(Box::new(ValueType::Array(Box::new(inner_type)))),
                    false => ValueType::Array(Box::new(inner_type)),
                }
            }
        }
    }

    pub fn compute_update_type(&self) -> ValueType {
        todo!()
        // return both flatten types
        // ignore readonlys
    }
}

impl TableParsed {
    pub fn compute_create_fields(&self) -> HashMap<String, ValueType> {
        let mut fields = HashMap::new();
        for (key, field) in &self.fields {
            if !field.has_override_value {
                fields.insert(key.clone(), field.compute_create_type());
            }
        }
        fields
    }

    pub fn compute_select_fields(&self) -> HashMap<String, ValueType> {
        let mut fields = HashMap::new();
        for (key, value) in &self.fields {
            fields.insert(key.clone(), value.compute_select_type());
        }
        fields
    }

    pub fn compute_update_fields(&self) -> HashMap<String, ValueType> {
        let mut fields = HashMap::new();
        for (key, value) in &self.fields {
            fields.insert(key.clone(), value.compute_update_type());
        }
        fields
    }
}

fn parse_table(
    table: &DefineTableStatement,
    field_definitions: &Vec<(Idiom, DefineFieldStatement)>,
) -> anyhow::Result<TableParsed> {
    let mut fields = HashMap::from([(
        "id".into(),
        FieldParsed {
            name: "id".into(),
            is_optional: false,
            field_type: FieldType::Simple,
            has_default: true,
            has_override_value: false,
            readonly: true,
            flexible: false,
            return_type: ValueType::Record(vec![table.name.clone().into()]),
        },
    )]);

    for (idiom, field) in field_definitions {
        let return_type = match &field.kind {
            Some(kind) => kind_to_return_type(kind)?,
            None => anyhow::bail!("You must define a type for field `{}`", field.to_string()),
        };

        let to_insert = FieldParsed {
            name: match &idiom[idiom.len() - 1] {
                Part::Field(ident) => ident.to_string(),
                _ => anyhow::bail!("Invalid path `{}`", idiom),
            },
            is_optional: return_type.is_optional(),
            field_type: match &return_type {
                ValueType::Any => FieldType::NestedObject(HashMap::new()),
                ValueType::Option(box ValueType::Any) => FieldType::NestedObject(HashMap::new()),
                _ => FieldType::Simple,
            },
            has_default: field.default.is_some(),
            has_override_value: match &field.value {
                Some(value) => !value_uses_value_param(value)?,
                None => false,
            },
            readonly: field.readonly,
            flexible: field.flex,
            return_type: match &return_type {
                ValueType::Option(inner_type) => *inner_type.clone(),
                _ => return_type,
            },
        };

        insert_into_object(&idiom, &mut fields, to_insert)?;
    }

    return Ok(TableParsed {
        name: table.name.to_string(),
        fields,
    });
}

fn value_uses_value_param(value: &Value) -> Result<bool, anyhow::Error> {
    Ok(match value {
        Value::Param(Param { 0: ident, .. }) => ident.0 == "value",
        Value::Array(array) => {
            for value in array.iter() {
                if value_uses_value_param(value)? {
                    return Ok(true);
                }
            }
            false
        }
        Value::Object(object) => {
            for (_, value) in object.iter() {
                if value_uses_value_param(value)? {
                    return Ok(true);
                }
            }
            false
        }
        Value::Expression(box Expression::Binary { l, r, .. }) => {
            value_uses_value_param(l)? || value_uses_value_param(r)?
        }
        Value::Expression(box Expression::Unary { v, .. }) => value_uses_value_param(v)?,
        Value::Bool(_)
        | Value::Number(_)
        | Value::None
        | Value::Null
        | Value::Constant(_)
        | Value::Bytes(_)
        | Value::Idiom(_)
        | Value::Regex(_)
        | Value::Strand(_)
        | Value::Thing(_) => false,
        Value::Block(box block) => block_uses_value_param(block)?,
        Value::Query(query) => query_uses_value_param(query)?,
        Value::Function(box function) => function_uses_value_param(function)?,
        v => anyhow::bail!("Unsupported value type `{}`", v),
    })
}

fn block_uses_value_param(block: &Block) -> Result<bool, anyhow::Error> {
    for value in block.iter() {
        if entry_uses_value_param(value)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn function_uses_value_param(function: &Function) -> Result<bool, anyhow::Error> {
    match function {
        Function::Normal(_, values) | Function::Custom(_, values) | Function::Script(_, values) => {
            for value in values.iter() {
                if value_uses_value_param(value)? {
                    return Ok(true);
                }
            }
        }
        _ => anyhow::bail!("Unsupported function type"),
    }
    Ok(false)
}

fn entry_uses_value_param(entry: &Entry) -> Result<bool, anyhow::Error> {
    Ok(match entry {
        Entry::Value(value) => value_uses_value_param(value)?,
        Entry::Throw(ThrowStatement { error, .. }) => value_uses_value_param(error)?,
        Entry::Continue(_) | Entry::Break(_) => false,
        Entry::Ifelse(IfelseStatement { close, exprs, .. }) => {
            match close {
                Some(close) => {
                    if value_uses_value_param(close)? {
                        return Ok(true);
                    }
                }
                None => {}
            };

            for (v1, v2) in exprs.iter() {
                if value_uses_value_param(v1)? || value_uses_value_param(v2)? {
                    return Ok(true);
                }
            }
            false
        }
        Entry::Create(_) => anyhow::bail!("Create statements not supported in VALUE clause yet"),
        Entry::Update(_) => anyhow::bail!("Update statements not supported in VALUE clause yet"),
        Entry::Delete(_) => anyhow::bail!("Delete statements not supported in VALUE clause yet"),
        Entry::Relate(_) => anyhow::bail!("Relate statements not supported in VALUE clause yet"),
        Entry::Insert(_) => anyhow::bail!("Insert statements not supported in VALUE clause yet"),
        Entry::Output(_) => anyhow::bail!("Output statements not supported in VALUE clause yet"),
        Entry::Set(_) => anyhow::bail!("LET $var statements not supported in VALUE clause yet"),
        Entry::Select(_) => anyhow::bail!("Select statements not supported in VALUE clause yet"),
        Entry::Foreach(_) => anyhow::bail!("Foreach statements not supported in VALUE clause yet"),
        Entry::Upsert(_) => anyhow::bail!("Upsert statements not supported in VALUE clause yet"),
        Entry::Define(_) => anyhow::bail!("Define statements not supported in VALUE clause yet"),
        Entry::Remove(_) => anyhow::bail!("Remove statements not supported in VALUE clause yet"),
        Entry::Rebuild(_) => anyhow::bail!("Rebuild statements not supported in VALUE clause yet"),
        _ => todo!(),
    })
}

fn query_uses_value_param(query: &Query) -> Result<bool, anyhow::Error> {
    Ok(match query {
        _ => anyhow::bail!("Query expressions not supported in VALUE clause"),
    })
}

pub fn parse_schema(schema: &str) -> Result<SchemaParsed, anyhow::Error> {
    let statements = parse(schema)?.0;

    struct TableInfo {
        definition: DefineTableStatement,
        fields: Vec<(Idiom, DefineFieldStatement)>,
    }

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
                            TableInfo {
                                definition: table.clone(),
                                fields: Vec::new(),
                            },
                        );
                    }
                }
            }
            Statement::Define(DefineStatement::Field(field)) => {
                let table = match tables.get_mut(&field.what.to_string()) {
                    Some(table) => table,
                    None => {
                        anyhow::bail!(
                            "You tried to define a field on a table that hasn't been defined: `{}`",
                            field.to_string()
                        );
                    }
                };

                table.fields.push((field.name.clone(), field));
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
                            .collect::<Result<Vec<(String, ValueType)>, anyhow::Error>>()?,
                        block: block.clone(),
                    },
                );
            }
            // ignore other statements
            _ => {}
        }
    }

    let tables = {
        let mut new_tables = HashMap::new();
        for (name, table) in tables.iter_mut() {
            new_tables.insert(name.clone(), parse_table(&table.definition, &table.fields)?);
        }
        new_tables
    };

    return Ok(SchemaParsed {
        tables,
        functions,
        views,
    });
}

fn insert_into_object(
    idiom: &[Part],
    fields: &mut HashMap<String, FieldParsed>,
    field: FieldParsed,
) -> anyhow::Result<()> {
    // if the idiom is empty, we're at the end of the path
    if idiom.len() == 1 {
        match &idiom[0] {
            Part::Field(ident) => fields.insert(ident.to_string(), field),
            _ => anyhow::bail!("Invalid path `{}`", Idiom::from(idiom)),
        };

        return Ok(());
    }
    let next_part = idiom.first().unwrap();
    match next_part {
        Part::Field(field_ident) => match fields.get_mut(field_ident.as_str()) {
            Some(FieldParsed {
                field_type: FieldType::NestedObject(fields),
                ..
            }) => insert_into_object(idiom[1..].as_ref(), fields, field),
            _ => anyhow::bail!("Field `{}` is not a nested object", field_ident),
        },
        // Part::All(index) => match fields.get_mut(index) {
        //     Some(FieldInfo {
        //         field_type: FieldType::NestedArray(array_type),
        //         ..
        //     }) => insert_into_array_type(idiom[1..].as_ref(), array_type, field),
        //     _ => anyhow::bail!("Index `{}` is not a nested array", index),
        // },
        _ => anyhow::bail!("Invalid path `{}`", Idiom::from(idiom)),
    }
}
