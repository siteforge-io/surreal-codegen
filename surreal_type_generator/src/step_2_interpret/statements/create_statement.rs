use surrealdb::sql::{statements::CreateStatement, Data, Fields, Output, Value, Values};

use crate::{
    kind,
    step_2_interpret::{get_statement_fields, schema::QueryState, utils::get_value_table},
    Kind,
};

pub fn get_create_statement_return_type(
    create: &CreateStatement,
    state: &mut QueryState,
) -> Result<Kind, anyhow::Error> {
    let is_only = create.only;

    let return_type = match &create.output {
        // default return type
        Some(Output::After) | None => get_create_fields(create, state, None)?,
        Some(Output::Before | Output::Null) => Kind::Null,
        Some(Output::None) => Kind::Null,
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
        Ok(kind!(Arr return_type))
    }
}

fn get_create_fields(
    create: &CreateStatement,
    state: &mut QueryState,
    fields: Option<&Fields>,
) -> Result<Kind, anyhow::Error> {
    get_statement_fields(&create.what, state, fields, |fields, state| {
        state.set_local("after", kind!(Obj fields.clone()));
        state.set_local("before", Kind::Null);
        state.set_local("this", kind!(Obj fields.clone()));
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
                let table_name = get_value_table(table, state)?;
                match state.schema.schema.tables.get(&table_name) {
                    Some(table) => {
                        let create_fields = kind!(Obj table.compute_create_fields()?);
                        tables
                            .push(kind!(Either [create_fields.clone(), kind!(Arr create_fields)]));
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
                state.infer(&param.0.as_str(), Kind::Either(tables));
            }

            Ok(())
        }
        // TODO: support other data types and variable inference
        _ => Ok(()),
    }
}
