use std::collections::BTreeMap;

use surrealdb::sql::{
    parse,
    statements::{
        DefineFieldStatement, DefineFunctionStatement, DefineStatement, DefineTableStatement,
        IfelseStatement, ThrowStatement,
    },
    Block, Entry, Expression, Fields, Function, Idiom, Kind, Param, Part, Query, Statement, Tables,
    Value,
};

use crate::kind;

#[derive(Debug, PartialEq)]
pub struct SchemaParsed {
    pub tables: BTreeMap<String, TableParsed>,
    pub functions: BTreeMap<String, FunctionParsed>,
    pub views: BTreeMap<String, ViewParsed>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ViewParsed {
    pub name: String,
    pub expr: Fields,
    pub what: Tables,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParsed {
    pub name: String,
    pub arguments: Vec<(String, Kind)>,
    pub block: Block,
}

#[derive(Debug, PartialEq)]

pub enum FieldType {
    Simple,
    NestedObject(BTreeMap<String, FieldParsed>),
    NestedArray(Box<FieldType>),
}

#[derive(Debug, PartialEq)]
pub struct TableParsed {
    pub name: String,
    pub id_value_type: Kind,
    pub fields: BTreeMap<String, FieldParsed>,
}

#[derive(Debug, PartialEq)]
pub struct FieldParsed {
    pub name: String,
    pub is_optional: bool,
    pub return_type: Kind,
    pub has_default: bool,
    pub has_override_value: bool,
    pub flexible: bool,
    pub readonly: bool,
    pub field_type: FieldType,
}

impl FieldParsed {
    pub fn compute_create_type(&self) -> anyhow::Result<Kind> {
        Ok(match &self.field_type {
            FieldType::Simple => match self.is_optional || self.has_default {
                true => Kind::Option(Box::new(self.return_type.clone())),
                false => self.return_type.clone(),
            },
            FieldType::NestedObject(obj) => {
                let mut fields = BTreeMap::new();
                for (key, value) in obj {
                    if !value.has_override_value {
                        fields.insert(key.clone(), value.compute_create_type()?);
                    }
                }

                let fields = kind!(Obj fields);

                match self.is_optional || self.has_default {
                    true => Kind::Option(Box::new(fields)),
                    false => fields,
                }
            }
            _ => anyhow::bail!("TODO: Unsupported field type: {:?}", self.field_type),
        })
    }

    pub fn compute_select_type(&self) -> anyhow::Result<Kind> {
        Ok(match &self.field_type {
            FieldType::Simple => match self.is_optional {
                true => Kind::Option(Box::new(self.return_type.clone())),
                false => self.return_type.clone(),
            },
            FieldType::NestedObject(obj) => {
                let mut fields = BTreeMap::new();
                for (key, value) in obj {
                    fields.insert(key.clone(), value.compute_select_type()?);
                }

                let fields = kind!(Obj fields);

                match self.is_optional {
                    true => Kind::Option(Box::new(fields)),
                    false => fields,
                }
            }
            FieldType::NestedArray(box inner_type) => {
                let select_type = match inner_type {
                    FieldType::Simple => self.return_type.clone(),
                    FieldType::NestedObject(fields) => {
                        let mut select_fields = BTreeMap::new();
                        for (key, value) in fields {
                            select_fields.insert(key.clone(), value.compute_select_type()?);
                        }
                        kind!(Obj select_fields)
                    }
                    FieldType::NestedArray(..) => {
                        anyhow::bail!("Nested array in nested array are not yet supported")
                    }
                };

                match self.is_optional {
                    true => kind!(Opt(kind!([select_type]))),
                    false => kind!(Arr select_type),
                }
            }
        })
    }

    pub fn compute_update_type(&self) -> anyhow::Result<Kind> {
        anyhow::bail!("TODO: query interpretation for UPDATE statements is not yet supported")
        // return both flatten types
        // ignore readonlys
    }
}

impl TableParsed {
    pub fn compute_create_fields(&self) -> anyhow::Result<BTreeMap<String, Kind>> {
        let mut fields = BTreeMap::new();
        for (key, field) in &self.fields {
            if !field.has_override_value {
                fields.insert(key.clone(), field.compute_create_type()?);
            }
        }
        Ok(fields)
    }

    pub fn compute_select_fields(&self) -> anyhow::Result<BTreeMap<String, Kind>> {
        let mut fields = BTreeMap::new();
        for (key, value) in &self.fields {
            fields.insert(key.clone(), value.compute_select_type()?);
        }
        Ok(fields)
    }

    pub fn compute_update_fields(&self) -> anyhow::Result<BTreeMap<String, Kind>> {
        let mut fields = BTreeMap::new();
        for (key, value) in &self.fields {
            fields.insert(key.clone(), value.compute_update_type()?);
        }
        Ok(fields)
    }
}

fn parse_table(
    table: &DefineTableStatement,
    field_definitions: &Vec<(Idiom, DefineFieldStatement)>,
) -> anyhow::Result<TableParsed> {
    // insert the implicit id field
    let mut fields = BTreeMap::from([(
        "id".into(),
        FieldParsed {
            name: "id".into(),
            is_optional: false,
            field_type: FieldType::Simple,
            has_default: true,
            has_override_value: false,
            readonly: true,
            flexible: false,
            return_type: Kind::Record(vec![table.name.clone().into()]),
        },
    )]);

    for (idiom, field) in field_definitions {
        let return_type = match &field.kind {
            Some(kind) => kind,
            None => &kind!(Any),
        };

        let mut to_insert = FieldParsed {
            name: match &idiom[idiom.len() - 1] {
                Part::Field(ident) => ident.to_string(),
                _ => anyhow::bail!("Invalid path `{}`", idiom),
            },
            is_optional: match return_type {
                Kind::Option(..) => true,
                _ => false,
            },
            field_type: match &return_type {
                Kind::Any => match field.flex {
                    false => FieldType::NestedObject(BTreeMap::new()),
                    true => FieldType::Simple,
                },
                Kind::Option(box Kind::Any) => match field.flex {
                    false => FieldType::NestedObject(BTreeMap::new()),
                    true => FieldType::Simple,
                },
                Kind::Array(box Kind::Any, ..) => match field.flex {
                    false => {
                        FieldType::NestedArray(Box::new(FieldType::NestedObject(BTreeMap::new())))
                    }
                    true => FieldType::Simple,
                },
                _ => FieldType::Simple,
            },
            has_default: field.default.is_some(),
            has_override_value: match &field.value {
                Some(value) => !value_uses_value_param(value)?,
                None => false,
            },
            readonly: field.readonly,
            flexible: field.flex,
            return_type: match return_type {
                Kind::Option(inner_type) => *inner_type.clone(),
                _ => return_type.clone(),
            },
        };

        // Handle edge case where `DEFINE FIELD id ON foo TYPE string` must have a default value
        if idiom.len() == 1 && idiom[0] == Part::Field("id".into()) {
            to_insert.has_default = true;
        }

        insert_into_object(&idiom, &mut fields, to_insert)?;
    }

    // Handle edge case where DEFINE FIELD id ON foo TYPE string is used
    // Since, the return type is still a record<foo> we need to note that.
    let id_value_type = match &mut fields.get_mut("id").unwrap().return_type {
        Kind::Record(..) => Kind::String,
        val => {
            let id_value_type = val.clone();
            *val = Kind::Record(vec![table.name.clone().into()]);
            id_value_type
        }
    };

    return Ok(TableParsed {
        name: table.name.to_string(),
        id_value_type,
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
        _ => anyhow::bail!("Unsupported statement type: `{}`", entry),
    })
}

fn query_uses_value_param(_query: &Query) -> Result<bool, anyhow::Error> {
    anyhow::bail!("Query expressions not supported in VALUE clause")
    // Ok(match query {
    //     #[allow(unreachable_patterns)]
    //     _ => anyhow::bail!("Query expressions not supported in VALUE clause"),
    // })
}

pub fn parse_schema(schema: &str) -> Result<SchemaParsed, anyhow::Error> {
    let statements = parse(schema)?.0;

    struct TableInfo {
        definition: DefineTableStatement,
        fields: Vec<(Idiom, DefineFieldStatement)>,
    }

    let mut tables = BTreeMap::new();
    let mut views = BTreeMap::new();
    let mut functions = BTreeMap::new();

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
                            .map(|(ident, kind)| Ok((ident.to_string(), kind.clone())))
                            .collect::<Result<Vec<(String, Kind)>, anyhow::Error>>()?,
                        block: block.clone(),
                    },
                );
            }
            // ignore other statements
            _ => {}
        }
    }

    let tables = {
        let mut new_tables = BTreeMap::new();
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
    fields: &mut BTreeMap<String, FieldParsed>,
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
            Some(FieldParsed {
                field_type: FieldType::NestedArray(array_type),
                ..
            }) => insert_into_array_type(idiom[1..].as_ref(), array_type, field),
            _ => anyhow::bail!("Field `{}` is not a nested object or array", field_ident),
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

fn insert_into_array_type(
    idiom: &[Part],
    array_type: &mut FieldType,
    field: FieldParsed,
) -> anyhow::Result<()> {
    // if the idiom is empty, we're at the end of the path

    match idiom.first().unwrap() {
        // eg: foo.*.bar
        Part::All => match array_type {
            FieldType::NestedObject(fields) => {
                insert_into_object(idiom[1..].as_ref(), fields, field)
            }
            _ => anyhow::bail!("Unimplemented path `{}`", Idiom::from(idiom)),
        },
        _ => anyhow::bail!("Unimplemented path `{}`", Idiom::from(idiom)),
    }
}
