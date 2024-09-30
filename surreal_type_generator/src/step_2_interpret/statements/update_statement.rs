use surrealdb::sql::{statements::UpdateStatement, Data, Fields, Output, Value, Values};

use crate::{
    step_2_interpret::{get_statement_fields, schema::QueryState},
    ValueType,
};

pub fn get_update_statement_return_type(
    update: &UpdateStatement,
    state: &mut QueryState,
) -> Result<ValueType, anyhow::Error> {
    let is_only = update.only;

    let return_type = match &update.output {
        Some(Output::After) | None => get_update_fields(update, state, None)?,
        Some(Output::Before) => ValueType::Either(vec![
            get_update_fields(update, state, None)?,
            ValueType::Null,
        ]),
        Some(Output::Null) => ValueType::Null,
        Some(Output::Diff) => Err(anyhow::anyhow!("Update with returned diff not supported"))?,
        Some(Output::Fields(fields)) => get_update_fields(update, state, Some(fields))?,
        Some(Output::None) => ValueType::Never,
        #[allow(unreachable_patterns)]
        _ => Err(anyhow::anyhow!(format!(
            "Unknown UPDATE statement type: {}",
            update
        )))?,
    };

    match &update.data {
        Some(content) => validate_data_type(state, &update.what, &content)?,
        None => {}
    }

    if is_only {
        Ok(return_type)
    } else {
        Ok(ValueType::Array(Box::new(return_type)))
    }
}

fn get_update_fields(
    update: &UpdateStatement,
    state: &mut QueryState,
    fields: Option<&Fields>,
) -> Result<ValueType, anyhow::Error> {
    get_statement_fields(&update.what, state, fields, |fields, state| {
        state.set_local("after", ValueType::Object(fields.clone()));
        state.set_local(
            "before",
            ValueType::Either(vec![ValueType::Object(fields.clone()), ValueType::Null]),
        );
        state.set_local("this", ValueType::Object(fields.clone()));
    })
}

fn validate_data_type(
    state: &mut QueryState,
    what: &Values,
    data: &Data,
) -> Result<(), anyhow::Error> {
    match data {
        Data::SetExpression(sets) => {
            for set in sets.iter() {
                // TODO

                // let mut tables = vec![];

                // for table in what.iter() {
                //     let table_name = match &table {
                //         Value::Table(table) => table.0.as_str(),
                //         _ => anyhow::bail!("Expected table name"),
                //     };
                // }

                // if tables.len() == 1 {
                //     state.infer(param.0.as_str(), tables.pop().unwrap());
                // } else if tables.len() > 1 {
                //     state.infer(&param.0.as_str(), ValueType::Either(tables));
                // }
            }

            Ok(())
        }
        _ => Err(anyhow::anyhow!(
            "Unsupported data type for UPDATE statement"
        ))?,
    }
}
