use surrealdb::sql::{statements::InsertStatement, Data, Fields, Output, Param, Value, Values};

use crate::{
    step_2_interpret::{get_statement_fields, schema::QueryState},
    ValueType,
};

pub fn get_insert_statement_return_type(
    insert: &InsertStatement,
    state: &mut QueryState,
) -> Result<ValueType, anyhow::Error> {
    let table = match &insert.into {
        Some(table @ Value::Table(..)) => table,
        _ => anyhow::bail!("Expected table name"),
    };

    let return_type = match &insert.output {
        Some(Output::After) | None => get_insert_fields(&[table.clone()], state, None)?,
        Some(Output::Before | Output::Null) => ValueType::Null,
        Some(Output::None) => ValueType::Never,
        Some(Output::Diff) => anyhow::bail!("Insert with returned diff is not currently supported"),
        Some(Output::Fields(fields)) => get_insert_fields(&[table.clone()], state, Some(fields))?,
        #[allow(unreachable_patterns)]
        _ => anyhow::bail!("Unknown INSERT statement type: {}", insert),
    };

    validate_data_type(state, &[table.clone()], &insert.data)?;

    Ok(ValueType::Array(Box::new(return_type)))
}

fn get_insert_fields(
    values: &[Value],
    state: &mut QueryState,
    fields: Option<&Fields>,
) -> Result<ValueType, anyhow::Error> {
    get_statement_fields(&values, state, fields, |fields, state| {
        state.set_local(
            "after",
            ValueType::Array(Box::new(ValueType::Object(fields.clone()))),
        );
        state.set_local("before", ValueType::Array(Box::new(ValueType::Null)));
        state.set_local(
            "this",
            ValueType::Array(Box::new(ValueType::Object(fields.clone()))),
        );
    })
}

fn validate_data_type(
    state: &mut QueryState,
    values: &[Value],
    data: &Data,
) -> Result<(), anyhow::Error> {
    match data {
        Data::SingleExpression(Value::Param(param)) => {
            let mut tables = vec![];

            for value in values {
                let table_name = match &value {
                    Value::Table(table) => table.0.as_str(),
                    _ => anyhow::bail!("Expected table name"),
                };

                match state.schema.schema.tables.get(table_name) {
                    Some(table) => {
                        let insert_fields = ValueType::Object(table.compute_create_fields());

                        // can insert multiple or a single record
                        tables.push(ValueType::Either(
                            [
                                ValueType::Array(Box::new(insert_fields.clone())),
                                insert_fields,
                            ]
                            .into(),
                        ))
                    }
                    None => anyhow::bail!(
                        "Trying to insert a record into a non-existent table: {}",
                        table_name
                    ),
                }
            }

            if tables.len() == 1 {
                state.infer(&param.0.as_str(), tables.pop().unwrap());
            } else if tables.len() > 1 {
                state.infer(&param.0.as_str(), ValueType::Either(tables));
            }

            Ok(())
        }
        // TODO: Support other types of data and variable inference
        _ => Ok(()),
    }
}
