use crate::QueryReturnType;

pub struct TypeData {
    pub name: String,
    pub types: String,
}

pub fn generate_header_typescript() -> String {
    let mut output = String::new();
    // output.push_str("// eslint-disable-next-line @typescript-eslint/ban-ts-comment\n");
    // output.push_str("// @ts-nocheck\n");
    output.push_str("import { type RecordId, Surreal } from 'surrealdb.js';\n");

    output
}

pub fn generate_typescript_output(types: &[TypeData]) -> String {
    let mut output = String::new();

    for TypeData { types, .. } in types {
        output.push_str(&types);
        output.push_str("\n");
    }

    output.push_str("export class TypedSurreal extends Surreal {\n");

    for TypeData { name, .. } in types {
        output.push_str(&format!(
            "    typed(query: typeof {}Query, variables: {}QueryVariables): Promise<{}QueryResult>;\n",
            name, name, name
        ));
    }

    output.push_str(
        "    typed(query: string, variables: Record<string, unknown>): Promise<unknown[]> {\n",
    );
    output.push_str("        return super.query(query, variables);\n");
    output.push_str("    }\n");

    output.push_str("};\n");

    output
}

pub fn generate_typescript_file(
    file_name: &str,
    query: &str,
    schema: &str,
) -> Result<TypeData, anyhow::Error> {
    // let (query, castings) = crate::step_1_parse_sql::parse_query(query)?;
    // let schema_query = crate::step_1_parse_sql::parse_sql(schema)?;
    // let tables = crate::step_1_parse_sql::get_tables(&schema_query)?;
    // let return_types = crate::step_2_interpret_query::interpret_query(&query, &tables, &castings)?;

    let (parameters, return_types, query) =
        crate::step_3_outputs::query_to_return_type(query, schema)?;

    let camel_case_file_name = filename_to_camel_case(file_name)?;

    let mut output = String::new();
    let query_string = serde_json::to_string(&query.to_string())?;
    output.push_str(&format!(
        "export const {}Query = {}\n",
        camel_case_file_name, query_string
    ));
    output.push_str(&format!(
        "export type {}QueryResult = [",
        camel_case_file_name
    ));

    for return_type in return_types {
        output.push_str(&generate_type_definition(return_type)?);
        output.push_str(",");
    }

    output.push_str("];\n");

    output.push_str(&format!(
        "export type {}QueryVariables = {{\n",
        camel_case_file_name
    ));

    for (name, return_type) in parameters {
        output.push_str(&format!(
            "    {}: {},\n",
            name,
            generate_type_definition(return_type)?
        ));
    }

    output.push_str("}\n");

    Ok(TypeData {
        name: camel_case_file_name,
        types: output,
    })
}

fn generate_type_definition(return_type: QueryReturnType) -> Result<String, anyhow::Error> {
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
            let string = generate_type_definition(*array)?;
            Ok(format!("{}[]", string))
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
            let string = generate_type_definition(*optional_value)?;
            Ok(format!("{}|null", string))
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
