use std::{collections::HashMap, sync::Arc};

use surrealdb::sql::Statements;

use crate::{step_2_interpret::SchemaState, ValueType};

pub struct TypeData {
    pub name: String,
    pub statements: Statements,
    pub return_type: Vec<ValueType>,
    pub variables: HashMap<String, ValueType>,
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
                output.push_str(&generate_type_definition(return_type, false)?);
                output.push_str(",");
            }
            output
        }));

        if variables.len() > 0 {
            output.push_str(&format!("export type {}Variables = {{", name));

            output.push_str(&generate_type_definition(
                &ValueType::Object(variables.clone()),
                true,
            )?);

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
    state: Arc<SchemaState>,
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

fn generate_type_definition(
    return_type: &ValueType,
    use_optional_keys: bool,
) -> Result<String, anyhow::Error> {
    match return_type {
        ValueType::Any => Ok("any".to_string()),
        ValueType::Number => Ok("number".to_string()),
        ValueType::Never => Ok("never".to_string()),
        ValueType::Null => Ok("null".to_string()),
        ValueType::Unknown => Ok("unknown".to_string()),
        ValueType::String => Ok("string".to_string()),
        ValueType::Int => Ok("number".to_string()),
        ValueType::Float => Ok("number".to_string()),
        ValueType::Datetime => Ok("Date".to_string()),
        ValueType::Duration => Ok("Duration".to_string()),
        ValueType::Decimal => Ok("Decimal".to_string()),
        ValueType::Bool => Ok("boolean".to_string()),
        ValueType::Uuid => Ok("string".to_string()),
        ValueType::Object(map) => {
            let mut output = String::new();
            output.push_str("{");

            // sort alphabetically for deterministic output
            let mut map: Vec<(_, _)> = map.into_iter().collect();
            map.sort_by_key(|x| x.0.to_string());

            for (key, value) in map {
                output.push_str(&format!(
                    "{}{}:{},",
                    key,
                    if use_optional_keys { "?" } else { "" },
                    generate_type_definition(value, use_optional_keys)?
                ));
            }

            output.push_str("}");
            Ok(output)
        }
        ValueType::Array(array) => {
            let string = generate_type_definition(&**array, use_optional_keys)?;
            Ok(format!("Array<{}>", string))
        }
        ValueType::Either(vec) => {
            let mut output = String::new();
            output.push_str("(");

            for return_type in vec.into_iter() {
                output.push_str("|");
                output.push_str(&generate_type_definition(return_type, use_optional_keys)?);
            }

            output.push_str(")");
            Ok(output)
        }
        ValueType::Record(tables) => {
            let mut output = String::new();
            output.push_str("RecordId<");

            for table in tables.iter() {
                output.push_str(&format!(" |'{}'", table.0));
            }

            output.push_str(">");
            Ok(output)
        }
        ValueType::Option(optional_value) => {
            let string = generate_type_definition(&**optional_value, use_optional_keys)?;
            Ok(format!("{}", string))
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
