#![feature(box_patterns)]

pub mod step_1_parse_sql;
pub mod step_2_interpret;
pub mod step_3_codegen;
pub use step_3_codegen::QueryResult;
pub use surrealdb::sql::Kind;
pub use surrealdb::sql::{Duration, Literal, Number};
pub mod utils;

pub use utils::printing::type_info_to_string;
pub use utils::printing::PrettyString;

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
