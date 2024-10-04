use crate::{kind, Kind};
use std::{collections::BTreeMap, sync::Arc};
use surrealdb::sql::{Literal, Statements, Table};

use crate::step_2_interpret::SchemaState;

pub struct TypeData {
    pub schema: Arc<SchemaState>,
    pub name: String,
    pub statements: Statements,
    pub return_type: Vec<Kind>,
    pub variables: BTreeMap<String, Kind>,
}

pub fn generate_type_info(
    file_name: &str,
    query: &str,
    state: Arc<SchemaState>,
) -> Result<TypeData, anyhow::Error> {
    let result = crate::step_3_codegen::output_query_type(query, state.clone())?;
    let camel_case_file_name = filename_to_camel_case(file_name)?;

    Ok(TypeData {
        schema: state.clone(),
        name: camel_case_file_name,
        return_type: result.return_types,
        statements: {
            let mut s = Statements::default();
            s.0 = result.statements;
            s
        },
        variables: result.variables,
    })
}

pub fn generate_typescript_output(
    types: &[TypeData],
    header: &str,
) -> Result<String, anyhow::Error> {
    let mut output = String::new();

    output.push_str(header);
    output.push_str("\n");

    for TypeData {
        schema,
        name,
        statements,
        return_type,
        variables,
    } in types
    {
        output.push_str(&format!(
            "export const {}Query = {}\n",
            name,
            serde_json::to_string(&statements.to_string())?
        ));
        output.push_str(&format!("export type {}Result = [{}]\n", name, {
            let mut output = String::new();
            for return_type in return_type {
                output.push_str(&generate_type_definition(return_type, schema)?);
                output.push_str(",");
            }
            output
        }));

        if variables.len() > 0 {
            output.push_str(&format!("export type {}Variables = ", name));

            output.push_str(&generate_type_definition(
                &kind!(Obj variables.clone()),
                schema,
            )?);

            output.push_str("\n");
        }
    }

    output.push_str(&format!("export type Queries = {{\n{}}}\n", {
        let mut output = String::new();
        for TypeData {
            name, variables, ..
        } in types
        {
            let has_variables = variables.len() > 0;
            output.push_str(&format!(
                "    [{}Query]: {{variables: {}, result: {}Result }}\n",
                name,
                if has_variables {
                    format!("{}Variables", name)
                } else {
                    "never".into()
                },
                name,
            ));
        }
        output
    },));

    output.push_str(r#"

export type Variables<Q extends keyof Queries> = Queries[Q]["variables"] extends never ? [] : [Queries[Q]["variables"]]

export class TypedSurreal extends Surreal {
    typed<Q extends keyof Queries>(query: Q, ...rest: Variables<Q>): Promise<Queries[Q]["result"]> {
        return this.query(query, rest[0])
    }
}
"#);

    Ok(output)
}

fn get_table_id_type(table: &Table, schema: &SchemaState) -> Result<String, anyhow::Error> {
    let table_parsed = schema.schema.tables.get(table.0.as_str()).unwrap();
    generate_type_definition(&table_parsed.id_value_type, schema)
}

fn generate_type_definition(
    return_type: &Kind,
    schema: &SchemaState,
) -> Result<String, anyhow::Error> {
    match return_type {
        Kind::Any => Ok("any".to_string()),
        Kind::Number => Ok("number".to_string()),
        Kind::Null => Ok("null".to_string()),
        Kind::String => Ok("string".to_string()),
        Kind::Int => Ok("number".to_string()),
        Kind::Float => Ok("number".to_string()),
        Kind::Datetime => Ok("Date".to_string()),
        Kind::Duration => Ok("Duration".to_string()),
        Kind::Decimal => Ok("Decimal".to_string()),
        Kind::Bool => Ok("boolean".to_string()),
        Kind::Uuid => Ok("string".to_string()),
        Kind::Array(array, ..) => {
            let string = generate_type_definition(&**array, schema)?;
            Ok(format!("Array<{}>", string))
        }
        Kind::Either(vec) => {
            let mut output = String::new();
            output.push_str("(");

            for return_type in vec.into_iter() {
                output.push_str("|");
                output.push_str(&generate_type_definition(return_type, schema)?);
            }

            output.push_str(")");
            Ok(output)
        }
        Kind::Record(tables) => {
            let mut output = String::new();
            output.push_str("(RecordId<");

            let table_idents = tables
                .iter()
                .map(|table| format!("\"{}\"", table.0))
                .collect::<Vec<_>>();
            let tables_joined = table_idents.join(" | ");

            output.push_str(&tables_joined);

            output.push_str("> & { id: ");
            output.push_str(&get_table_id_type(tables.first().unwrap(), schema)?);
            output.push_str(" })");
            Ok(output)
        }
        Kind::Option(optional_value) => {
            let string = generate_type_definition(&**optional_value, schema)?;
            Ok(format!("{} | undefined", string))
        }
        Kind::Object => Ok("any".to_string()),

        // ========
        // Literals
        // ========
        Kind::Literal(Literal::String(string)) => Ok(serde_json::to_string(&string)?),
        Kind::Literal(Literal::Duration(_duration)) => Ok("Duration".to_string()),
        Kind::Literal(Literal::Number(number)) => Ok(number.to_string()),
        Kind::Literal(Literal::Object(map)) => {
            let mut output = String::new();
            output.push_str("{");

            // sort alphabetically for deterministic output
            let mut map: Vec<(_, _)> = map.into_iter().collect();
            map.sort_by_key(|x| x.0.to_string());

            for (key, value) in map {
                output.push_str(&format!(
                    "{}{}:{},",
                    key,
                    match value {
                        Kind::Option(_) => "?",
                        _ => "",
                    },
                    match value {
                        Kind::Option(inner) => generate_type_definition(inner, schema)?,
                        value => generate_type_definition(value, schema)?,
                    },
                ));
            }

            output.push_str("}");
            Ok(output)
        }
        Kind::Literal(Literal::Array(..)) => {
            anyhow::bail!("Literal::Array not yet supported")
            // let string = generate_type_definition(&**array, schema)?;
            // Ok(format!("Array<{}>", string))
        }

        // Catch all
        kind => unimplemented!("Kind {:?} not yet supported", kind),
    }
}

fn filename_to_camel_case(filename: &str) -> Result<String, anyhow::Error> {
    let parts: Vec<&str> = filename.split('.').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!(
            "Filename must be of the form `name.extension`"
        ));
    }

    let name_part = parts[0];
    let mut camel_case_name = String::new();
    let mut new_word = true;

    for c in name_part.chars() {
        if c == '_' {
            new_word = true;
        } else if new_word {
            camel_case_name.push(c.to_uppercase().next().unwrap());
            new_word = false;
        } else {
            camel_case_name.push(c);
        }
    }

    Ok(camel_case_name)
}
