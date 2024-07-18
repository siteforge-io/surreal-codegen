use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use surrealdb::sql::Statements;

use crate::{step_2_interpret::SchemaState, QueryReturnType};

pub struct TypeData {
    pub name: String,
    pub statements: Statements,
    pub return_type: Vec<QueryReturnType>,
    pub variables: HashMap<String, QueryReturnType>,
}

pub fn generate_typescript_output(types: &[TypeData]) -> Result<String, anyhow::Error> {
    let mut output = String::new();

    output.push_str("import { type RecordId, Surreal } from 'surrealdb.js';\n");

    for TypeData {
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
                output.push_str(&generate_type_definition(return_type)?);
                output.push_str(",");
            }
            output
        }));

        if variables.len() > 0 {
            output.push_str(&format!("export type {}Variables = {{", name));

            for (name, return_type) in variables {
                match return_type {
                    QueryReturnType::Option(return_type) => output.push_str(&format!(
                        "    {}?: {},",
                        name,
                        generate_type_definition(return_type)?
                    )),
                    _ => output.push_str(&format!(
                        "    {}: {},",
                        name,
                        generate_type_definition(return_type)?
                    )),
                };
            }

            output.push_str("}\n");
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

type QueryKeys = keyof Queries
type InferVariables<Q extends keyof Queries> = Queries[Q]["variables"]
type InferResult<Q extends keyof Queries> = Queries[Q]["result"]
type QueryWithVariables<Q extends QueryKeys> = InferVariables<Q> extends never ? Q : Q & string
type QueryWithoutVariables<Q extends QueryKeys> = InferVariables<Q> extends never ? Q : Exclude<Q, string>

Surreal.prototype.typed = function <Q extends keyof Queries, V extends InferVariables<Q>>(query: Q, variables?: V): Promise<InferResult<Q>> {
    return this.query(query, variables)
}

declare module "surrealdb.js" {
    interface Surreal {
        typed<Q extends QueryKeys>(query: QueryWithVariables<Q>, variables: InferVariables<Q>): Promise<InferResult<Q>>;
        typed<Q extends QueryKeys>(query: QueryWithoutVariables<Q>): Promise<InferResult<Q>>;
    }
}
"#);

    Ok(output)
}

pub fn generate_type_info(
    file_name: &str,
    query: &str,
    state: Arc<Mutex<SchemaState>>,
) -> Result<TypeData, anyhow::Error> {
    let result = crate::step_3_codegen::output_query_type(query, state)?;
    let camel_case_file_name = filename_to_camel_case(file_name)?;

    Ok(TypeData {
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

fn generate_type_definition(return_type: &QueryReturnType) -> Result<String, anyhow::Error> {
    match return_type {
        QueryReturnType::Any => Ok("any".to_string()),
        QueryReturnType::Number => Ok("number".to_string()),
        QueryReturnType::Never => Ok("never".to_string()),
        QueryReturnType::Null => Ok("null".to_string()),
        QueryReturnType::Unknown => Ok("unknown".to_string()),
        QueryReturnType::String => Ok("string".to_string()),
        QueryReturnType::Int => Ok("number".to_string()),
        QueryReturnType::Float => Ok("number".to_string()),
        QueryReturnType::Datetime => Ok("Date".to_string()),
        QueryReturnType::Duration => Ok("Duration".to_string()),
        QueryReturnType::Decimal => Ok("Decimal".to_string()),
        QueryReturnType::Bool => Ok("boolean".to_string()),
        QueryReturnType::Uuid => Ok("string".to_string()),
        QueryReturnType::Object(map) => {
            let mut output = String::new();
            output.push_str("{");

            // sort alphabetically for deterministic output
            let mut map: Vec<(_, _)> = map.into_iter().collect();
            map.sort_by_key(|x| x.0.to_string());

            for (key, value) in map {
                output.push_str(&format!("{}:{},", key, generate_type_definition(value)?));
            }

            output.push_str("}");
            Ok(output)
        }
        QueryReturnType::Array(array) => {
            let string = generate_type_definition(&**array)?;
            Ok(format!("Array<{}>", string))
        }
        QueryReturnType::Either(vec) => {
            let mut output = String::new();
            output.push_str("(");

            for return_type in vec.into_iter() {
                output.push_str("|");
                output.push_str(&generate_type_definition(return_type)?);
            }

            output.push_str(")");
            Ok(output)
        }
        QueryReturnType::Record(tables) => {
            let mut output = String::new();
            output.push_str("RecordId<");

            for table in tables.iter() {
                output.push_str(&format!(" |'{}'", table.0));
            }

            output.push_str(">");
            Ok(output)
        }
        QueryReturnType::Option(optional_value) => {
            let string = generate_type_definition(&**optional_value)?;
            Ok(format!("{}|undefined", string))
        }
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
