use crate::QueryReturnType;

pub fn generate_typescript(
    file_name: &str,
    query: &str,
    schema: &str,
) -> Result<String, anyhow::Error> {
    let (query, castings) = crate::step_1_parse_sql::parse_query(query)?;
    let schema_query = crate::step_1_parse_sql::parse_sql(schema)?;
    let tables = crate::step_1_parse_sql::get_tables(&schema_query)?;
    let return_types = crate::step_2_interpret_query::interpret_query(&query, &tables, &castings)?;

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

    output.push_str("]");

    Ok(output)
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
        QueryReturnType::Record(_) => unimplemented!(),
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
