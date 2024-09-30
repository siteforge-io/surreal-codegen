#![feature(box_patterns)]

pub mod step_1_parse_sql;
pub mod step_2_interpret;
pub mod step_3_codegen;
pub use step_3_codegen::QueryResult;
pub use surrealdb::sql::Kind;
pub use surrealdb::sql::{Duration, Literal, Number};

#[macro_export]
macro_rules! var_map {
    {$($key:tt : $value:expr),* $(,)?} => {
        {
            #[allow(unused_mut)]
            let mut map = std::collections::BTreeMap::new();
            $(
                map.insert(stringify!($key).to_string(), $value);
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! kind {

    // Match simple kinds by identifier.
    (Any) => { $crate::Kind::Any };
    (Null) => { $crate::Kind::Null };
    (Bool) => { $crate::Kind::Bool };
    (Bytes) => { $crate::Kind::Bytes };
    (Datetime) => { $crate::Kind::Datetime };
    (Decimal) => { $crate::Kind::Decimal };
    (Duration) => { $crate::Kind::Duration };
    (Float) => { $crate::Kind::Float };
    (Int) => { $crate::Kind::Int };
    (Number) => { $crate::Kind::Number };
    (Object) => { $crate::Kind::Object };
    (Point) => { $crate::Kind::Point };
    (String) => { $crate::Kind::String };
    (Uuid) => { $crate::Kind::Uuid };
    (Range) => { $crate::Kind::Range };
    (Record [$($table:tt),+ $(,)?]) => {
        $crate::Kind::Record(vec![$($table.into()),+])
    };

    // Match Array literal with one element as Kind::Array(Box::new(kind!($elem)))
    ([ $expr:expr ]) => {
        $crate::Kind::Array(Box::new($expr), None)
    };

    // Match array literals with elements that are expressions.
    // [ $($elem:expr),* $(,)? ] => {
    //     $crate::Kind::Literal($crate::Literal::Array(vec![$($elem),*]))
    // };

    // Match object literals with values that are expressions.
    ({ $($key:tt : $value:expr),* $(,)? }) => {
        $crate::Kind::Literal($crate::Literal::Object({
            #[allow(unused_mut)]
            let mut map = std::collections::BTreeMap::new();
            $(
                let key_str = kind!(@key_to_string $key);
                map.insert(key_str, kind!($value));
            )*
            map
        }))
    };

    // Type where expr is a hashmap
    (Obj $expr:expr) => {
        $crate::Kind::Literal($crate::Literal::Object($expr))
    };

    // Type where expr is a vector
    (Arr $expr:expr) => {
        $crate::Kind::Array(Box::new($expr), None)
    };

    // Recursive case for `Option` with expression support.
    (Opt($inner:expr)) => {
        $crate::Kind::Option(Box::new($inner))
    };

    // Recursive case for `Either`.
    (Either[$($inner:expr),+ $(,)?]) => {
        $crate::Kind::Either(vec![$(kind!($inner)),+])
    };

    // Cases for `Set` with and without size.
    (Set[$($inner:tt)+]) => {
        $crate::kind_set!($($inner)+)
    };

    // Helper to parse function arguments.
    (@parse_args [$($args:expr),*]) => {
        $(kind!($args)),*
    };

    // Helper to convert keys to strings.
    (@key_to_string $key:ident) => { stringify!($key).to_string() };
    (@key_to_string $key:literal) => { $key.to_string() };

    // Fallback case for expressions.
    ($e:expr) => { $e };
}

#[macro_export]
macro_rules! kind_set {
    ($kind:expr, $size:expr) => {
        $crate::Kind::Set(Box::new(kind!($kind)), Some($size))
    };
    ($kind:expr) => {
        $crate::Kind::Set(Box::new(kind!($kind)), None)
    };
}

#[macro_export]
macro_rules! kind_array {
    ($kind:expr, $size:expr) => {
        $crate::Kind::Array(Box::new(kind!($kind)), Some($size))
    };
    ($kind:expr) => {
        $crate::Kind::Array(Box::new(kind!($kind)), None)
    };
}

// use std::collections::BTreeMap;
// use surrealdb::sql::{Duration, Kind, Literal, Number, Table};

// #[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
// pub enum Kind {
//     Any,                                 // yes
//     Never,                               // nope, could replace with null
//     Null,                                // yes
//     Bool,                                // yees
//     Duration,                            // yes
//     Decimal,                             // yes
//     Datetime,                            // yes
//     String,                              // yes
//     Int,                                 // yes
//     Float,                               // yes
//     Number,                              // yes
//     Uuid,                                // yes
//     Object(BTreeMap<String, Kind>), // yes, Literal::Object
//     Array(Box<Kind>),               // yes, Literal::Array
//     Either(Vec<Kind>),              // yes
//     Record(Vec<Table>),                  // yes
//     Option(Box<Kind>),              // yes

//     // Literals
//     StringLiteral(String), // Literal::String
//     NumberLiteral(Number), // Literal::Number
//     DurationLiteral(Duration), // Literal::Duration
//                            // TOOD: Sets
//                            // TODO: Geometries
// }

// impl Kind {
//     pub fn expect_option(self) -> Result<Kind, anyhow::Error> {
//         match self {
//             Kind::Option(return_type) => Ok(*return_type),
//             _ => anyhow::bail!("Expected an option type, but got: {:?}", self),
//         }
//     }

//     pub fn is_optional(&self) -> bool {
//         match self {
//             Kind::Option(_) => true,
//             _ => false,
//         }
//     }
// }

// pub fn kind_to_return_type(kind: &Kind) -> Result<Kind, anyhow::Error> {
//     Ok(match kind {
//         Kind::Any => Kind::Any,
//         Kind::Null => Kind::Null,
//         Kind::String => Kind::String,
//         Kind::Int => Kind::Int,
//         Kind::Float => Kind::Float,
//         Kind::Datetime => Kind::Datetime,
//         Kind::Duration => Kind::Duration,
//         Kind::Decimal => Kind::Decimal,
//         Kind::Bool => Kind::Bool,
//         Kind::Number => Kind::Number,
//         Kind::Record(tables) => Kind::Record(tables.clone()),
//         Kind::Option(kind) => Kind::Option(Box::new(kind_to_return_type(kind)?)),
//         Kind::Uuid => Kind::Uuid,
//         Kind::Array(kind, _) => Kind::Array(Box::new(kind_to_return_type(kind)?)),
//         Kind::Object => Kind::Any,
//         Kind::Literal(literal) => match literal {
//             Literal::String(s) => Kind::StringLiteral(s.0.clone()),
//             Literal::Number(n) => Kind::NumberLiteral(n.clone()),
//             Literal::Duration(d) => Kind::DurationLiteral(d.clone()),
//             Literal::Object(obj) => {
//                 let mut fields = BTreeMap::new();
//                 for (key, value) in obj {
//                     fields.insert(key.into(), kind_to_return_type(value)?);
//                 }
//                 Kind::Object(fields)
//             }
//             Literal::Array(values) => {
//                 let mut eithers = Vec::new();
//                 for value in values {
//                     eithers.push(kind_to_return_type(value)?);
//                 }
//                 if eithers.len() == 1 {
//                     Kind::Array(Box::new(eithers.into_iter().next().unwrap()))
//                 } else {
//                     Kind::Array(Box::new(Kind::Either(eithers)))
//                 }
//             }
//             _ => anyhow::bail!("Unknown literal: {:?}", literal),
//         },
//         Kind::Point => anyhow::bail!("Points are not yet supported"),
//         Kind::Bytes => anyhow::bail!("Bytes is not yet supported"),
//         Kind::Geometry(_) => anyhow::bail!("Geometry is not yet supported"),
//         Kind::Set(kind, _) => Kind::Array(Box::new(kind_to_return_type(kind)?)),
//         Kind::Either(kinds) => {
//             let mut types = Vec::new();
//             for kind in kinds {
//                 types.push(kind_to_return_type(kind)?);
//             }
//             Kind::Either(types)
//         }
//         #[allow(unreachable_patterns)]
//         _ => anyhow::bail!("Unknown kind: {:?}", kind),
//     })
// }
