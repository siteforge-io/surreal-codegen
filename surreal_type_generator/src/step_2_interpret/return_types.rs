use std::collections::{BTreeMap, HashSet};

use surrealdb::sql::{
    Cast, Constant, Expression, Field, Fields, Ident, Idiom, Literal, Operator, Param, Part, Value,
};

use crate::{kind, Kind};

use super::{
    function::get_function_return_type,
    get_subquery_return_type,
    object::get_object_return_type,
    schema::QueryState,
    utils::{get_what_fields, merge_into_map_recursively},
};

pub fn get_statement_fields<F>(
    what: &[Value],
    state: &mut QueryState,
    fields: Option<&Fields>,
    get_field_and_variables: F,
) -> Result<Kind, anyhow::Error>
where
    F: Fn(&mut BTreeMap<String, Kind>, &mut QueryState) -> (),
{
    let mut return_types = Vec::new();
    let mut used_tables = HashSet::new();

    for table in what.iter() {
        let mut table_fields = get_what_fields(table, state)?;

        if used_tables.contains(&table) {
            continue;
        }

        used_tables.insert(table);

        state.push_stack_frame();

        let return_type = if let Some(fields) = fields {
            get_field_and_variables(&mut table_fields, state);
            get_fields_return_values(fields, &table_fields, state)?
        } else {
            kind!(Obj table_fields.clone())
        };

        state.pop_stack_frame();

        return_types.push(return_type);
    }

    if return_types.len() == 1 {
        Ok(return_types.pop().unwrap())
    } else {
        Ok(Kind::Either(return_types))
    }
}

pub fn get_fields_return_values(
    fields: &Fields,
    field_types: &BTreeMap<String, Kind>,
    state: &mut QueryState,
) -> Result<Kind, anyhow::Error> {
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
            let mut map = BTreeMap::new();

            for field in fields {
                for (idiom, return_type) in get_field_return_type(field, &field_types, state)? {
                    merge_into_map_recursively(&mut map, &idiom.0, return_type)?;
                }
            }

            return Ok(kind!(Obj map));
        }
    }
}

pub fn get_field_return_type(
    field: &Field,
    field_types: &BTreeMap<String, Kind>,
    state: &mut QueryState,
) -> Result<Vec<(Idiom, Kind)>, anyhow::Error> {
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
    field_types: &BTreeMap<String, Kind>,
    state: &mut QueryState,
) -> Result<Kind, anyhow::Error> {
    Ok(match expr {
        Value::Idiom(idiom) => get_field_from_paths(&idiom.0, &field_types, state)?,
        Value::Subquery(subquery) => {
            state.push_stack_frame();

            state.set_local("parent", kind!(Obj field_types.clone()));

            let return_type = get_subquery_return_type(subquery, state)?;

            state.pop_stack_frame();

            return_type
        }
        Value::Param(param) => get_parameter_return_type(param, state)?,
        // TODO: These constants could potentially be represented as actual constants in the return types
        Value::Strand(_) => Kind::String,
        Value::Number(_) => Kind::Number,
        Value::Bool(_) => Kind::Bool,
        Value::Null => Kind::Null,
        Value::Datetime(_) => Kind::Datetime,
        Value::Duration(_) => Kind::Duration,
        Value::None => Kind::Null,
        Value::Function(func) => get_function_return_type(state, &func)?,
        Value::Expression(expr) => get_expression_return_type(expr, field_types, state)?,
        Value::Array(array) => {
            let mut return_types = HashSet::new();
            for value in &array.0 {
                return_types.insert(get_value_return_type(value, field_types, state)?);
            }
            // If there is more than one type, we muse use Either
            kind!(Arr match return_types.len() {
                0 => Kind::Null,
                1 => return_types.into_iter().next().unwrap(),
                _ => Kind::Either(return_types.into_iter().collect()),
            })
        }
        Value::Object(obj) => get_object_return_type(state, obj)?,
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
            | Constant::TimeEpoch => Kind::Number,
            _ => anyhow::bail!("Unsupported constant: {:?}", constant),
        },
        Value::Cast(box Cast { 0: kind, .. }) => kind.clone(),
        _ => anyhow::bail!("Unsupported value/expression: {}", expr),
    })
}

pub fn get_expression_return_type(
    expr: &Expression,
    field_types: &BTreeMap<String, Kind>,
    state: &mut QueryState,
) -> Result<Kind, anyhow::Error> {
    Ok(match expr {
        // Unary
        Expression::Unary {
            o: Operator::Not, ..
        } => Kind::Bool,
        Expression::Unary {
            o: Operator::Neg, ..
        } => anyhow::bail!("Unsupported unary operator"),

        // logical binary expressions
        Expression::Binary {
            o: Operator::And, ..
        } => Kind::Bool,
        Expression::Binary {
            o: Operator::Or, ..
        } => Kind::Bool,
        Expression::Binary {
            o: Operator::Equal, ..
        } => Kind::Bool,
        Expression::Binary {
            o: Operator::NotEqual,
            ..
        } => Kind::Bool,
        Expression::Binary {
            o: Operator::Exact, ..
        } => Kind::Bool,

        // comparison binary expressions
        Expression::Binary {
            o: Operator::LessThan,
            ..
        } => Kind::Bool,
        Expression::Binary {
            o: Operator::MoreThan,
            ..
        } => Kind::Bool,
        Expression::Binary {
            o: Operator::LessThanOrEqual,
            ..
        } => Kind::Bool,
        Expression::Binary {
            o: Operator::MoreThanOrEqual,
            ..
        } => Kind::Bool,
        Expression::Binary {
            o: Operator::Like, ..
        } => Kind::Bool,
        Expression::Binary {
            o: Operator::NotLike,
            ..
        } => Kind::Bool,

        // TODO: arithmetic
        Expression::Binary {
            l,
            o: Operator::Add,
            r,
        } => {
            let l = get_value_return_type(l, field_types, state)?;
            let r = get_value_return_type(r, field_types, state)?;

            match (&l, &r) {
                (Kind::Number, Kind::Number) => Kind::Number,
                (Kind::String, Kind::String) => Kind::String,
                (Kind::Datetime, Kind::Datetime) => Kind::Datetime,
                (Kind::Duration, Kind::Duration) => Kind::Duration,
                _ => anyhow::bail!("Unsupported binary operation: {:?}", expr),
            }
        }
        // Expression
        // TODO: short circuiting
        // TODO: more (contains, any, etc, outside, inside, fuzzy match)
        _ => anyhow::bail!("Unsupported expression: {}", expr),
    })
}

pub fn get_parameter_return_type(
    param: &Param,
    state: &mut QueryState,
) -> Result<Kind, anyhow::Error> {
    match state.get(&param.0 .0) {
        Some(return_type) => Ok(return_type.clone()),
        None => anyhow::bail!("Unknown parameter: {}", param),
    }
}

pub fn get_field_from_paths(
    parts: &[Part],
    field_types: &BTreeMap<String, Kind>,
    state: &mut QueryState,
) -> Result<Kind, anyhow::Error> {
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
        Some(Part::Start(Value::Subquery(subquery))) => {
            let return_type = get_subquery_return_type(subquery, state)?;
            match_return_type(&return_type, &parts[1..], field_types, state)
        }
        Some(Part::All) => Ok(kind!(Obj field_types.clone())),
        Some(_) => anyhow::bail!("Unsupported path: {}", Idiom::from(parts)),
        // Some(_) => anyhow::bail!("Unsupported path: {:#?}", parts),
        // We're returning an actual object
        None => Ok(kind!(Obj field_types.clone())),
    }
}

fn match_return_type(
    return_type: &Kind,
    parts: &[Part],
    field_types: &BTreeMap<String, Kind>,
    state: &mut QueryState,
) -> Result<Kind, anyhow::Error> {
    let has_next_part = parts.len() > 1;

    Ok(match return_type {
        Kind::String => Kind::String,
        Kind::Int => Kind::Int,
        Kind::Float => Kind::Float,
        Kind::Datetime => Kind::Datetime,
        Kind::Duration => Kind::Duration,
        Kind::Decimal => Kind::Decimal,
        Kind::Bool => Kind::Bool,
        Kind::Null => Kind::Null,
        Kind::Uuid => Kind::Uuid,
        Kind::Any => Kind::Any,
        Kind::Number => Kind::Number,
        Kind::Object => Kind::Object,
        Kind::Literal(lit @ (Literal::String(_) | Literal::Number(_))) => {
            Kind::Literal(lit.clone())
        }
        Kind::Literal(Literal::Object(nested_fields)) => {
            if has_next_part {
                get_field_from_paths(&parts[1..], nested_fields, state)?
            } else {
                kind!(Obj nested_fields.clone())
            }
        }
        Kind::Record(tables) => {
            if has_next_part {
                let mut return_types = Vec::new();
                for table in tables.iter() {
                    let return_type = get_field_from_paths(
                        &parts[1..],
                        &state.table_select_fields(table.as_str())?,
                        state,
                    )?;
                    return_types.push(return_type);
                }
                if return_types.len() == 1 {
                    return_types.pop().unwrap()
                } else {
                    Kind::Either(return_types)
                }
            } else {
                Kind::Record(tables.clone())
            }
        }
        Kind::Option(return_type) => kind!(Opt(match_return_type(
            return_type,
            &parts,
            field_types,
            state,
        )?)),
        Kind::Array(return_type, ..) => match parts.first() {
            Some(Part::Index(_)) => Kind::Option(Box::new(match_return_type(
                return_type,
                &parts,
                field_types,
                state,
            )?)),
            Some(Part::All) => kind!(Arr match_return_type(
            return_type,
            &parts,
            field_types,
            state,
        )?),
            Some(Part::Field(_)) => kind!(Arr match_return_type(
            return_type,
            &parts[1..],
            field_types,
            state,
        )?),
            Some(_) => return Err(anyhow::anyhow!("Unsupported path: {}", Idiom::from(parts))),
            None => {
                return Err(anyhow::anyhow!(
                    "Tried to access array with no fields: {}",
                    Idiom::from(parts)
                ))
            }
        },
        Kind::Either(return_types) => {
            let mut return_types = return_types.clone();
            for return_type in &mut return_types {
                *return_type = match_return_type(&return_type, &parts, field_types, state)?;
            }
            Kind::Either(return_types)
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported return type: {:?}",
                return_type
            ))
        }
    })
}
