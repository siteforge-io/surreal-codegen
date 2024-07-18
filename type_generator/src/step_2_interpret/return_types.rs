use std::collections::{HashMap, HashSet};

use surrealdb::sql::{
    Cast, Constant, Expression, Field, Fields, Ident, Idiom, Operator, Param, Part, Value,
};

use crate::{kind_to_return_type, QueryReturnType};

use super::{
    function::get_function_return_type,
    get_subquery_return_type,
    schema::QueryState,
    utils::{get_what_table, merge_into_map_recursively},
};

pub fn get_statement_fields<F>(
    what: &[Value],
    state: &mut QueryState,
    fields: Option<&Fields>,
    get_field_and_variables: F,
) -> Result<QueryReturnType, anyhow::Error>
where
    F: Fn(&mut HashMap<String, QueryReturnType>, &mut QueryState) -> (),
{
    let mut return_types = Vec::new();
    let mut used_tables = HashSet::new();

    for value in what.iter() {
        let table = get_what_table(value, state)?;

        if used_tables.contains(&table.name) {
            continue;
        }
        used_tables.insert(table.name.clone());

        let return_type = if let Some(fields) = fields {
            state.push_stack_frame();
            let mut table_fields = table.fields.clone();

            get_field_and_variables(&mut table_fields, state);

            get_fields_return_values(fields, &table_fields, state)?
        } else {
            QueryReturnType::Object(table.fields.clone())
        };

        return_types.push(return_type);
    }

    if return_types.len() == 1 {
        Ok(return_types.pop().unwrap())
    } else {
        Ok(QueryReturnType::Either(return_types))
    }
}

pub fn get_fields_return_values(
    fields: &Fields,
    field_types: &HashMap<String, QueryReturnType>,
    state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    match fields {
        // returning a single value with `VALUE`
        Fields {
            0: fields, 1: true, ..
        } => Ok(get_field_return_type(&fields[0], &field_types, state)?
            .pop()
            .unwrap()
            .1),
        // returning multiple values (object map)
        Fields {
            0: fields,
            1: false,
            ..
        } => {
            let mut map = HashMap::new();

            for field in fields {
                for (idiom, return_type) in get_field_return_type(field, &field_types, state)? {
                    merge_into_map_recursively(&mut map, &idiom.0, return_type)?;
                }
            }

            return Ok(QueryReturnType::Object(map));
        }
    }
}

pub fn get_field_return_type(
    field: &Field,
    field_types: &HashMap<String, QueryReturnType>,
    state: &mut QueryState,
) -> Result<Vec<(Idiom, QueryReturnType)>, anyhow::Error> {
    match field {
        Field::Single {
            expr,
            alias: Some(alias),
        } => Ok(vec![(
            alias.clone(),
            get_value_return_type(expr, field_types, state)?,
        )]),
        Field::Single { expr, alias: None } => Ok(vec![(
            expr.to_idiom(),
            get_value_return_type(expr, field_types, state)?,
        )]),
        Field::All => {
            let mut results = vec![];
            for (field_name, field_type) in field_types {
                results.push((
                    vec![Part::Field(Ident::from(field_name.clone()))].into(),
                    field_type.clone(),
                ));
            }
            Ok(results)
        }
        #[allow(unreachable_patterns)]
        _ => anyhow::bail!("Unsupported field: {}", field),
    }
}

pub fn get_value_return_type(
    expr: &Value,
    field_types: &HashMap<String, QueryReturnType>,
    state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    Ok(match expr {
        Value::Idiom(idiom) => get_field_from_paths(&idiom.0, &field_types, state)?,
        Value::Subquery(subquery) => {
            state.push_stack_frame();

            state.set_local("parent", QueryReturnType::Object(field_types.clone()));

            let return_type = get_subquery_return_type(subquery, state)?;

            state.pop_stack_frame();

            return_type
        }
        Value::Param(param) => get_parameter_return_type(param, state)?,
        // These constants could potentially be represented as actual constants in the return types
        Value::Strand(_) => QueryReturnType::String,
        Value::Number(_) => QueryReturnType::Number,
        Value::Bool(_) => QueryReturnType::Bool,
        Value::Null => QueryReturnType::Null,
        Value::Datetime(_) => QueryReturnType::Datetime,
        Value::Duration(_) => QueryReturnType::Duration,
        Value::None => QueryReturnType::Null,
        Value::Function(func) => get_function_return_type(state, &func)?,
        Value::Expression(expr) => get_expression_return_type(expr, field_types, state)?,
        Value::Array(_) => anyhow::bail!("Arrays are not yet supported"),
        Value::Object(_) => anyhow::bail!("Objects are not yet supported"),
        Value::Constant(constant) => match constant {
            Constant::MathE
            | Constant::MathFrac1Pi
            | Constant::MathFrac1Sqrt2
            | Constant::MathFrac2Pi
            | Constant::MathFrac2SqrtPi
            | Constant::MathFracPi2
            | Constant::MathFracPi3
            | Constant::MathFracPi4
            | Constant::MathFracPi6
            | Constant::MathFracPi8
            | Constant::MathInf
            | Constant::MathLn10
            | Constant::MathLn2
            | Constant::MathLog102
            | Constant::MathLog10E
            | Constant::MathLog210
            | Constant::MathLog2E
            | Constant::MathPi
            | Constant::MathSqrt2
            | Constant::MathTau
            | Constant::TimeEpoch => QueryReturnType::Number,
            _ => anyhow::bail!("Unsupported constant: {:?}", constant),
        },
        Value::Cast(box Cast { 0: kind, .. }) => kind_to_return_type(kind)?,
        _ => anyhow::bail!("Unsupported value/expression: {}", expr),
    })
}

pub fn get_expression_return_type(
    expr: &Expression,
    _field_types: &HashMap<String, QueryReturnType>,
    mut _state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    Ok(match expr {
        // Unary
        Expression::Unary {
            o: Operator::Not, ..
        } => QueryReturnType::Bool,
        Expression::Unary {
            o: Operator::Neg, ..
        } => anyhow::bail!("Unsupported unary operator"),

        // logical binary expressions
        Expression::Binary {
            o: Operator::And, ..
        } => QueryReturnType::Bool,
        Expression::Binary {
            o: Operator::Or, ..
        } => QueryReturnType::Bool,
        Expression::Binary {
            o: Operator::Equal, ..
        } => QueryReturnType::Bool,
        Expression::Binary {
            o: Operator::NotEqual,
            ..
        } => QueryReturnType::Bool,
        Expression::Binary {
            o: Operator::Exact, ..
        } => QueryReturnType::Bool,

        // comparison binary expressions
        Expression::Binary {
            o: Operator::LessThan,
            ..
        } => QueryReturnType::Bool,
        Expression::Binary {
            o: Operator::MoreThan,
            ..
        } => QueryReturnType::Bool,
        Expression::Binary {
            o: Operator::LessThanOrEqual,
            ..
        } => QueryReturnType::Bool,
        Expression::Binary {
            o: Operator::MoreThanOrEqual,
            ..
        } => QueryReturnType::Bool,
        Expression::Binary {
            o: Operator::Like, ..
        } => QueryReturnType::Bool,
        Expression::Binary {
            o: Operator::NotLike,
            ..
        } => QueryReturnType::Bool,

        // TODO: arithmetic
        // Expression
        // TODO: short circuiting
        // TODO: more (contains, any, etc, outside, inside, fuzzy match)
        _ => anyhow::bail!("Unsupported expression: {}", expr),
    })
}

pub fn get_parameter_return_type(
    param: &Param,
    state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    match state.get(&param.0 .0) {
        Some(return_type) => Ok(return_type.clone()),
        None => anyhow::bail!("Unknown parameter: {}", param),
    }
}

pub fn get_field_from_paths(
    parts: &[Part],
    field_types: &HashMap<String, QueryReturnType>,
    state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    match parts.first() {
        Some(Part::Field(field_name)) => match field_types.get(field_name.as_str()) {
            Some(return_type) => match_return_type(return_type, &parts, field_types, state),
            None => anyhow::bail!("Field not found: {}", field_name),
        },
        Some(Part::Start(Value::Param(Param {
            0: Ident { 0: param_name, .. },
            ..
        }))) => match state.get(param_name) {
            Some(return_type) => {
                match_return_type(&return_type.clone(), &parts, field_types, state)
            }
            None => anyhow::bail!("Unknown parameter: {}", param_name),
        },
        Some(_) => anyhow::bail!("Unsupported path: {:#?}", parts),
        // We're returning an actual object
        None => Ok(QueryReturnType::Object(field_types.clone())),
    }
}

pub fn match_return_type(
    return_type: &QueryReturnType,
    parts: &[Part],
    field_types: &HashMap<String, QueryReturnType>,
    state: &mut QueryState,
) -> Result<QueryReturnType, anyhow::Error> {
    let has_next_part = parts.len() > 1;

    match return_type {
        QueryReturnType::Object(nested_fields) => {
            get_field_from_paths(&parts[1..], nested_fields, state)
        }
        QueryReturnType::String => Ok(QueryReturnType::String),
        QueryReturnType::Int => Ok(QueryReturnType::Int),
        QueryReturnType::Float => Ok(QueryReturnType::Float),
        QueryReturnType::Datetime => Ok(QueryReturnType::Datetime),
        QueryReturnType::Duration => Ok(QueryReturnType::Duration),
        QueryReturnType::Decimal => Ok(QueryReturnType::Decimal),
        QueryReturnType::Bool => Ok(QueryReturnType::Bool),
        QueryReturnType::Record(tables) => {
            if has_next_part {
                let mut return_types = Vec::new();
                for table in tables.iter() {
                    let return_type = get_field_from_paths(
                        &parts[1..],
                        &state.table(table.as_str())?.fields.clone(),
                        state,
                    )?;
                    return_types.push(return_type);
                }
                if return_types.len() == 1 {
                    Ok(return_types.pop().unwrap())
                } else {
                    Ok(QueryReturnType::Either(return_types))
                }
            } else {
                Ok(QueryReturnType::Record(tables.clone()))
            }
        }
        QueryReturnType::Option(return_type) => Ok(QueryReturnType::Option(Box::new(
            match_return_type(return_type, &parts, field_types, state)?,
        ))),
        QueryReturnType::Array(return_type) => Ok(QueryReturnType::Array(Box::new(
            match_return_type(return_type, &parts, field_types, state)?,
        ))),
        QueryReturnType::Null => Ok(QueryReturnType::Null),
        QueryReturnType::Uuid => Ok(QueryReturnType::Uuid),
        QueryReturnType::Any => Ok(QueryReturnType::Any),
        QueryReturnType::Number => Ok(QueryReturnType::Number),
        _ => anyhow::bail!("Unsupported return type: {:?}", return_type),
    }
}
