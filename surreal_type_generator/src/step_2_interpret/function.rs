use surrealdb::sql::{Function, Value};

use crate::ValueType;

use super::QueryState;

pub fn get_function_return_type(
    state: &mut QueryState,
    func: &Function,
) -> Result<ValueType, anyhow::Error> {
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
) -> Result<ValueType, anyhow::Error> {
    let function = state.function(name)?;

    Ok(function.return_type)
}

pub fn normal_function_return_type(name: &str) -> Result<ValueType, anyhow::Error> {
    Ok(match name {
        "count" => ValueType::Number,

        // `math::` functions
        "math::abs" => ValueType::Number,
        "math::acos" => ValueType::Number,
        "math::asin" => ValueType::Number,
        "math::atan" => ValueType::Number,
        "math::bottom" => ValueType::Array(Box::new(ValueType::Number)),
        "math::ceil" => ValueType::Number,
        "math::clamp" => ValueType::Number,
        "math::cos" => ValueType::Number,
        "math::cot" => ValueType::Number,
        "math::deg2rad" => ValueType::Number,
        "math::e" => ValueType::Number,
        "math::fixed" => ValueType::Number,
        "math::floor" => ValueType::Number,
        "math::inf" => ValueType::Number,
        "math::interquartile" => ValueType::Number,
        "math::lerp" => ValueType::Number,
        "math::lerpangle" => ValueType::Number,
        "math::ln" => ValueType::Number,
        "math::log" => ValueType::Number,
        "math::log10" => ValueType::Number,
        "math::log2" => ValueType::Number,
        "math::max" => ValueType::Number,
        "math::mean" => ValueType::Number,
        "math::median" => ValueType::Number,
        "math::midhinge" => ValueType::Number,
        "math::min" => ValueType::Number,
        "math::mode" => ValueType::Number,
        "math::nearestrank" => ValueType::Number,
        "math::neg_inf" => ValueType::Number,
        "math::percentile" => ValueType::Number,
        "math::pi" => ValueType::Number,
        "math::product" => ValueType::Number,
        "math::rad2deg" => ValueType::Number,
        "math::round" => ValueType::Number,
        "math::sign" => ValueType::Number,
        "math::sin" => ValueType::Number,
        "math::tan" => ValueType::Number,
        "math::tau" => ValueType::Number,
        "math::spread" => ValueType::Number,
        "math::sqrt" => ValueType::Number,
        "math::stddev" => ValueType::Number,
        "math::sum" => ValueType::Number,
        "math::top" => ValueType::Array(Box::new(ValueType::Number)),
        "math::trimean" => ValueType::Number,
        "math::variance" => ValueType::Number,

        // `time::` functions
        "time::day" => ValueType::Number,
        "time::floor" => ValueType::Number,
        "time::format" => ValueType::String,
        "time::group" => ValueType::Datetime,
        "time::hour" => ValueType::Number,
        "time::max" => ValueType::Datetime,
        "time::micros" => ValueType::Number,
        "time::millis" => ValueType::Number,
        "time::min" => ValueType::Datetime,
        "time::minute" => ValueType::Number,
        "time::month" => ValueType::Number,
        "time::nano" => ValueType::Number,
        "time::now" => ValueType::Datetime,
        "time::round" => ValueType::Datetime,
        "time::second" => ValueType::Number,
        "time::timezone" => ValueType::String,
        "time::unix" => ValueType::Number,
        "time::wday" => ValueType::Number,
        "time::week" => ValueType::Number,
        "time::yday" => ValueType::Number,
        "time::year" => ValueType::Number,
        "time::from::micros" => ValueType::Datetime,
        "time::from::millis" => ValueType::Datetime,
        "time::from::nanos" => ValueType::Datetime,
        "time::from::secs" => ValueType::Datetime,
        "time::from::unix" => ValueType::Datetime,

        // `duration::` functions
        "duration::days" => ValueType::Number,
        "duration::hours" => ValueType::Number,
        "duration::micros" => ValueType::Number,
        "duration::millis" => ValueType::Number,
        "duration::mins" => ValueType::Number,
        "duration::nanos" => ValueType::Number,
        "duration::secs" => ValueType::Number,
        "duration::weeks" => ValueType::Number,
        "duration::years" => ValueType::Number,
        "duration::from::days" => ValueType::Duration,
        "duration::from::hours" => ValueType::Duration,
        "duration::from::micros" => ValueType::Duration,
        "duration::from::millis" => ValueType::Duration,
        "duration::from::mins" => ValueType::Duration,
        "duration::from::nanos" => ValueType::Duration,
        "duration::from::secs" => ValueType::Duration,
        "duration::from::weeks" => ValueType::Duration,

        // `crypto::` functions
        "crypto::md5" => ValueType::String,
        "crypto::sha1" => ValueType::String,
        "crypto::sha256" => ValueType::String,
        "crypto::sha512" => ValueType::String,
        "crypto::argon2::compare" => ValueType::Bool,
        "crypto::argon2::generate" => ValueType::String,
        "crypto::bcrypt::compare" => ValueType::Bool,
        "crypto::bcrypt::generate" => ValueType::String,
        "crypto::pbkdf2::compare" => ValueType::Bool,
        "crypto::pbkdf2::generate" => ValueType::String,
        "crypto::scrypt::compare" => ValueType::Bool,
        "crypto::scrypt::generate" => ValueType::String,

        // `meta::` functions
        "meta::id" => ValueType::Any, // TODO: should this be a string?
        "meta::type" => ValueType::String,

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
