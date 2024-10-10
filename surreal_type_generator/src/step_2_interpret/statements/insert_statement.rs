use surrealdb::sql::{statements::InsertStatement, Data, Fields, Output, Value};

use crate::{
    kind,
    step_2_interpret::{get_statement_fields, schema::QueryState, utils::get_value_table},
    Kind,
};

pub fn get_insert_statement_return_type(
    insert: &InsertStatement,
    state: &mut QueryState,
) -> Result<Kind, anyhow::Error> {
    let into = match &insert.into {
        Some(into) => into,
        None => anyhow::bail!("Expected table name"),
    };

    let return_type = match &insert.output {
        Some(Output::After) | None => get_insert_fields(&into, state, None)?,
        Some(Output::Before | Output::Null) => Kind::Null,
        Some(Output::None) => Kind::Null,
        Some(Output::Diff) => anyhow::bail!("Insert with returned diff is not currently supported"),
        Some(Output::Fields(fields)) => get_insert_fields(&into, state, Some(fields))?,
        #[allow(unreachable_patterns)]
        _ => anyhow::bail!("Unknown INSERT statement type: {}", insert),
    };

    validate_data_type(state, &into, &insert.data)?;

    Ok(kind!(Arr return_type))
}

fn get_insert_fields(
    table: &Value,
    state: &mut QueryState,
    fields: Option<&Fields>,
) -> Result<Kind, anyhow::Error> {
    get_statement_fields(&[table.clone()], state, fields, |fields, state| {
        state.set_local("after", kind!(Obj fields.clone()));
        state.set_local("before", kind!(Null));
        state.set_local("this", kind!(Obj fields.clone()));
    })
}

fn validate_data_type(
    state: &mut QueryState,
    table: &Value,
    data: &Data,
) -> Result<(), anyhow::Error> {
    match data {
        Data::SingleExpression(Value::Param(param)) => {
            let mut tables = vec![];

            // for value in values {
            let table_name = get_value_table(&table, state)?;

            match state.schema.schema.tables.get(&table_name) {
                Some(table) => {
                    let insert_fields = kind!(Obj table.compute_create_fields());

                    // can insert multiple or a single record
                    tables.push(kind!(Either[kind!(Arr insert_fields.clone()), insert_fields]));
                }
                None => anyhow::bail!(
                    "Trying to insert a record into a non-existent table: {}",
                    table_name
                ),
            }
            // }

            if tables.len() == 1 {
                state.infer(&param.0.as_str(), tables.pop().unwrap());
            } else if tables.len() > 1 {
                state.infer(&param.0.as_str(), Kind::Either(tables));
            }

            Ok(())
        }
        // TODO: Support other types of data and variable inference
        _ => Ok(()),
    }
}
