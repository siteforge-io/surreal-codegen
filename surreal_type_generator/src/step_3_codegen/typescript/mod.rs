use crate::{kind, utils::printing::indent, Kind, PrettyString};
use surrealdb::sql::{Literal, Table};

use crate::step_2_interpret::SchemaState;

use super::TypeData;

pub fn format_comment(string: &str) -> String {
    let mut lines = Vec::new();
    lines.push("/**".into());
    for line in string.lines() {
        lines.push(format!(" * {}", line));
    }
    lines.push(" */".into());
    lines.join("\n")
}

pub fn generate_typescript_output(
    types: &[TypeData],
    header: &str,
) -> Result<String, anyhow::Error> {
    let mut output = String::new();

    colored::control::set_override(false);

    output.push_str(header);
    output.push_str("\n\n");

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

    for TypeData {
        schema,
        name,
        statements,
        return_type,
        variables,
    } in types
    {
        output.push_str(&format_comment(&format!(
            "## {} query results:\n\n```surql\n{}\n```",
            name,
            &return_type
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    format!(
                        "/// ---------------------\n{}{}:\n/// ---------------------\n{}",
                        "/// Result ",
                        i,
                        x.pretty_string()
                    )
                })
                .collect::<Vec<_>>()
                .join("\n\n"),
        )));
        output.push_str("\n");
        output.push_str(&format!(
            "export const {}Query = {}\n",
            name,
            serde_json::to_string(&statements.to_string())?
        ));
        output.push_str(&format!("export type {}Result = [\n{}\n]\n", name, {
            let mut lines = Vec::new();
            for result in return_type {
                lines.push(generate_type_definition(result, schema)?);
            }
            indent(&lines.join(",\n"))
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

    output.push_str(r#"

export type Variables<Q extends keyof Queries> = Queries[Q]["variables"] extends never ? [] : [Queries[Q]["variables"]]

/**
 * A Surreal client with typed queries from codegen.
 *
 * Usage:
 *
 * ```surql
 * // [your_schema_path].surql
 * DEFINE TABLE user SCHEMAFULL;
 * DEFINE FIELD name ON user TYPE string;
 * ```
 * ```surql
 * // queries/get_user.surql
 * SELECT * FROM ONLY $auth;
 * ```
 *
 * ```ts
 * // usage example
 * import { TypedSurreal, GetUserQuery } from "[YOUR_OUTPUT_PATH].ts"
 * const db = new TypedSurreal()
 *
 * await db.connect(...)
 *
 * const [
 *     user // { id: RecordId<"user">, name: string }
 * ] = await surreal.typed(GetUserQuery)
 *
 * console.log(user) // { id: 1, name: "John Doe" }
 * ```
 */
export class TypedSurreal extends Surreal {
    typed<Q extends keyof Queries>(query: Q, ...rest: Variables<Q>): Promise<Queries[Q]["result"]> {
        return this.query(query, rest[0])
    }
}
"#);

    Ok(output)
}

fn get_table_id_type(table: &Table, schema: &SchemaState) -> Result<String, anyhow::Error> {
    let table_parsed = match schema.schema.tables.get(table.0.as_str()) {
        Some(table) => table,
        None => anyhow::bail!("Tried to use a table that does't exist: {}", table.0),
    };
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
            output.push_str("(\n");

            let mut lines = Vec::new();

            for return_type in vec.into_iter() {
                lines.push(format!(
                    "| {}",
                    generate_type_definition(return_type, schema)?
                ));
            }

            output.push_str(&indent(&lines.join("\n")));

            output.push_str("\n)");
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
            output.push_str("{\n");

            // sort alphabetically for deterministic output
            let mut map: Vec<(_, _)> = map.into_iter().collect();
            map.sort_by_key(|x| x.0.to_string());

            let mut key_string = Vec::new();

            for (key, value) in map {
                key_string.push(format!(
                    "{}{}: {},\n",
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

            let key_string = indent(&key_string.join(""));

            output.push_str(&key_string);
            output.push_str("\n}");
            Ok(output)
        }
        Kind::Literal(Literal::Array(..)) => {
            anyhow::bail!("Literal::Array not yet supported")
            // let string = generate_type_definition(&**array, schema)?;
            // Ok(format!("Array<{}>", string))
        }

        // Catch all
        kind => anyhow::bail!("Kind {:?} not yet supported", kind),
    }
}
