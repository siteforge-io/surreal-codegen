use std::collections::HashMap;

use surrealdb::sql::{
    parse,
    statements::{
        DefineFieldStatement, DefineFunctionStatement, DefineStatement, DefineTableStatement,
    },
    Block, Fields, Idiom, Part, Statement, Tables,
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
    pub has_default_or_value: bool,
    pub flexible: bool,
    pub readonly: bool,
    pub field_type: FieldType,
}

impl FieldParsed {
    pub fn compute_create_type(&self) -> ValueType {
        match &self.field_type {
            FieldType::Simple => match self.is_optional || self.has_default_or_value {
                true => ValueType::Option(Box::new(self.return_type.clone())),
                false => self.return_type.clone(),
            },
            FieldType::NestedObject(obj) => {
                let mut fields = HashMap::new();
                for (key, value) in obj {
                    fields.insert(key.clone(), value.compute_create_type());
                }

                let fields = ValueType::Object(fields);

                match self.is_optional || self.has_default_or_value {
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
        for (key, value) in &self.fields {
            fields.insert(key.clone(), value.compute_create_type());
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
            has_default_or_value: true,
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

        println!("idiom: {:#?}", idiom);

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
            has_default_or_value: field.default.is_some() || field.value.is_some(), // TODO: check if uses variable expression internally
            readonly: field.readonly,
            flexible: field.flex,
            return_type: match &return_type {
                ValueType::Option(inner_type) => *inner_type.clone(),
                _ => return_type,
            },
        };

        println!("fields: {:#?}", to_insert);

        insert_into_object(&idiom, &mut fields, to_insert)?;
    }

    return Ok(TableParsed {
        name: table.name.to_string(),
        fields,
    });
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
