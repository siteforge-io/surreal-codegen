use surrealdb::sql::{Function, Value};

use crate::QueryReturnType;

use super::QueryState;

pub fn get_function_return_type(
    state: &mut QueryState,
    func: &Function,
) -> Result<QueryReturnType, anyhow::Error> {
    match func {
        Function::Custom(name, values) => get_custom_function_return_type(state, name, values),
        Function::Normal(name, ..) => normal_function_return_type(name),
        Function::Script(..) => anyhow::bail!("Script functions are not yet supported"),
        _ => anyhow::bail!("Unsupported function: {}", func),
    }
}

pub fn get_custom_function_return_type(
    state: &mut QueryState,
    name: &str,
    _values: &[Value],
) -> Result<QueryReturnType, anyhow::Error> {
    let function = state.function(name)?;

    Ok(function.return_type)
}

pub fn normal_function_return_type(name: &str) -> Result<QueryReturnType, anyhow::Error> {
    Ok(match name {
        "count" => QueryReturnType::Number,

        // `math::` functions
        "math::abs" => QueryReturnType::Number,
        "math::acos" => QueryReturnType::Number,
        "math::asin" => QueryReturnType::Number,
        "math::atan" => QueryReturnType::Number,
        "math::bottom" => QueryReturnType::Array(Box::new(QueryReturnType::Number)),
        "math::ceil" => QueryReturnType::Number,
        "math::clamp" => QueryReturnType::Number,
        "math::cos" => QueryReturnType::Number,
        "math::cot" => QueryReturnType::Number,
        "math::deg2rad" => QueryReturnType::Number,
        "math::e" => QueryReturnType::Number,
        "math::fixed" => QueryReturnType::Number,
        "math::floor" => QueryReturnType::Number,
        "math::inf" => QueryReturnType::Number,
        "math::interquartile" => QueryReturnType::Number,
        "math::lerp" => QueryReturnType::Number,
        "math::lerpangle" => QueryReturnType::Number,
        "math::ln" => QueryReturnType::Number,
        "math::log" => QueryReturnType::Number,
        "math::log10" => QueryReturnType::Number,
        "math::log2" => QueryReturnType::Number,
        "math::max" => QueryReturnType::Number,
        "math::mean" => QueryReturnType::Number,
        "math::median" => QueryReturnType::Number,
        "math::midhinge" => QueryReturnType::Number,
        "math::min" => QueryReturnType::Number,
        "math::mode" => QueryReturnType::Number,
        "math::nearestrank" => QueryReturnType::Number,
        "math::neg_inf" => QueryReturnType::Number,
        "math::percentile" => QueryReturnType::Number,
        "math::pi" => QueryReturnType::Number,
        "math::product" => QueryReturnType::Number,
        "math::rad2deg" => QueryReturnType::Number,
        "math::round" => QueryReturnType::Number,
        "math::sign" => QueryReturnType::Number,
        "math::sin" => QueryReturnType::Number,
        "math::tan" => QueryReturnType::Number,
        "math::tau" => QueryReturnType::Number,
        "math::spread" => QueryReturnType::Number,
        "math::sqrt" => QueryReturnType::Number,
        "math::stddev" => QueryReturnType::Number,
        "math::sum" => QueryReturnType::Number,
        "math::top" => QueryReturnType::Array(Box::new(QueryReturnType::Number)),
        "math::trimean" => QueryReturnType::Number,
        "math::variance" => QueryReturnType::Number,

        // `time::` functions
        "time::day" => QueryReturnType::Number,
        "time::floor" => QueryReturnType::Number,
        "time::format" => QueryReturnType::String,
        "time::group" => QueryReturnType::Datetime,
        "time::hour" => QueryReturnType::Number,
        "time::max" => QueryReturnType::Datetime,
        "time::micros" => QueryReturnType::Number,
        "time::millis" => QueryReturnType::Number,
        "time::min" => QueryReturnType::Datetime,
        "time::minute" => QueryReturnType::Number,
        "time::month" => QueryReturnType::Number,
        "time::nano" => QueryReturnType::Number,
        "time::now" => QueryReturnType::Datetime,
        "time::round" => QueryReturnType::Datetime,
        "time::second" => QueryReturnType::Number,
        "time::timezone" => QueryReturnType::String,
        "time::unix" => QueryReturnType::Number,
        "time::wday" => QueryReturnType::Number,
        "time::week" => QueryReturnType::Number,
        "time::yday" => QueryReturnType::Number,
        "time::year" => QueryReturnType::Number,
        "time::from::micros" => QueryReturnType::Datetime,
        "time::from::millis" => QueryReturnType::Datetime,
        "time::from::nanos" => QueryReturnType::Datetime,
        "time::from::secs" => QueryReturnType::Datetime,
        "time::from::unix" => QueryReturnType::Datetime,

        // `duration::` functions
        "duration::days" => QueryReturnType::Number,
        "duration::hours" => QueryReturnType::Number,
        "duration::micros" => QueryReturnType::Number,
        "duration::millis" => QueryReturnType::Number,
        "duration::mins" => QueryReturnType::Number,
        "duration::nanos" => QueryReturnType::Number,
        "duration::secs" => QueryReturnType::Number,
        "duration::weeks" => QueryReturnType::Number,
        "duration::years" => QueryReturnType::Number,
        "duration::from::days" => QueryReturnType::Duration,
        "duration::from::hours" => QueryReturnType::Duration,
        "duration::from::micros" => QueryReturnType::Duration,
        "duration::from::millis" => QueryReturnType::Duration,
        "duration::from::mins" => QueryReturnType::Duration,
        "duration::from::nanos" => QueryReturnType::Duration,
        "duration::from::secs" => QueryReturnType::Duration,
        "duration::from::weeks" => QueryReturnType::Duration,

        // `crypto::` functions
        "crypto::md5" => QueryReturnType::String,
        "crypto::sha1" => QueryReturnType::String,
        "crypto::sha256" => QueryReturnType::String,
        "crypto::sha512" => QueryReturnType::String,
        "crypto::argon2::compare" => QueryReturnType::Bool,
        "crypto::argon2::generate" => QueryReturnType::String,
        "crypto::bcrypt::compare" => QueryReturnType::Bool,
        "crypto::bcrypt::generate" => QueryReturnType::String,
        "crypto::pbkdf2::compare" => QueryReturnType::Bool,
        "crypto::pbkdf2::generate" => QueryReturnType::String,
        "crypto::scrypt::compare" => QueryReturnType::Bool,
        "crypto::scrypt::generate" => QueryReturnType::String,

        // `meta::` functions
        "meta::id" => QueryReturnType::Any, // TODO: should this be a string?
        "meta::type" => QueryReturnType::String,

        // TODO: add more functions
        // - `array::`
        // - `encoding::`
        // - `geo::`
        // - `http::`
        // - `object::`
        // - `parse::`
        // - `rand::`
        // - `search::`
        // - `session::`
        // - `sleep::`
        // - `string::`
        // - `type::`
        // - `vector::`
        // - ``
        _ => anyhow::bail!("Unsupported normal function: {}", name),
    })
}
