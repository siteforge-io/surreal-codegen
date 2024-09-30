use surrealdb::sql::{statements::CreateStatement, Data, Fields, Output, Value, Values};

use crate::{
    step_2_interpret::{get_statement_fields, schema::QueryState},
    ValueType,
};

pub fn get_create_statement_return_type(
    create: &CreateStatement,
    state: &mut QueryState,
) -> Result<ValueType, anyhow::Error> {
    let is_only = create.only;

    let return_type = match &create.output {
        // default return type
        Some(Output::After) | None => get_create_fields(create, state, None)?,
        Some(Output::Before | Output::Null) => ValueType::Null,
        Some(Output::None) => ValueType::Never,
        Some(Output::Diff) => anyhow::bail!("Create with returned diff is not currently supported"),
        Some(Output::Fields(fields)) => get_create_fields(create, state, Some(fields))?,
        #[allow(unreachable_patterns)]
        _ => anyhow::bail!("Unknown CREATE statement type: {}", create),
    };

    match &create.data {
        Some(content) => validate_data_type(state, &create.what, content)?,
        None => {}
    }

    if is_only {
        Ok(return_type)
    } else {
        Ok(ValueType::Array(Box::new(return_type)))
    }
}

fn get_create_fields(
    create: &CreateStatement,
    state: &mut QueryState,
    fields: Option<&Fields>,
) -> Result<ValueType, anyhow::Error> {
    get_statement_fields(&create.what, state, fields, |fields, state| {
        state.set_local("after", ValueType::Object(fields.clone()));
        state.set_local("before", ValueType::Null);
        state.set_local("this", ValueType::Object(fields.clone()));
    })
}

fn validate_data_type(
    state: &mut QueryState,
    what: &Values,
    data: &Data,
) -> Result<(), anyhow::Error> {
    match data {
        Data::ContentExpression(Value::Param(param)) => {
            // we want to infer the type of this param by reading the table's required types and fields for insertion
            let mut tables = Vec::new();

            for table in what.iter() {
                let table_name = match table {
                    Value::Table(table) => table.0.as_str(),
                    _ => anyhow::bail!("Expected table name"),
                };
                match state.schema.schema.tables.get(table_name) {
                    Some(table) => {
                        let create_fields = ValueType::Object(table.compute_create_fields());
                        tables.push(ValueType::Either(
                            [
                                ValueType::Array(Box::new(create_fields.clone())),
                                create_fields,
                            ]
                            .into(),
                        ));
                    }
                    None => anyhow::bail!(
                        "Trying to create a record with an unknown or view table: {}",
                        table_name
                    ),
                }
            }

            if tables.len() == 1 {
                state.infer(param.0.as_str(), tables.pop().unwrap());
            } else if tables.len() > 1 {
                state.infer(&param.0.as_str(), ValueType::Either(tables));
            }

            Ok(())
        }
        // TODO: support other data types and variable inference
        _ => Ok(()),
    }
}
