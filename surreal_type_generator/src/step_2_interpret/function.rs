use surrealdb::sql::{Function, Value};

use crate::Kind;

use super::QueryState;

pub fn get_function_return_type(
    state: &mut QueryState,
    func: &Function,
) -> Result<Kind, anyhow::Error> {
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
) -> Result<Kind, anyhow::Error> {
    let function = state.function(name)?;

    Ok(function.return_type)
}

pub fn normal_function_return_type(name: &str) -> Result<Kind, anyhow::Error> {
    Ok(match name {
        "count" => Kind::Number,

        // `math::` functions
        "math::abs" => Kind::Number,
        "math::acos" => Kind::Number,
        "math::asin" => Kind::Number,
        "math::atan" => Kind::Number,
        "math::bottom" => Kind::Array(Box::new(Kind::Number), None),
        "math::ceil" => Kind::Number,
        "math::clamp" => Kind::Number,
        "math::cos" => Kind::Number,
        "math::cot" => Kind::Number,
        "math::deg2rad" => Kind::Number,
        "math::e" => Kind::Number,
        "math::fixed" => Kind::Number,
        "math::floor" => Kind::Number,
        "math::inf" => Kind::Number,
        "math::interquartile" => Kind::Number,
        "math::lerp" => Kind::Number,
        "math::lerpangle" => Kind::Number,
        "math::ln" => Kind::Number,
        "math::log" => Kind::Number,
        "math::log10" => Kind::Number,
        "math::log2" => Kind::Number,
        "math::max" => Kind::Number,
        "math::mean" => Kind::Number,
        "math::median" => Kind::Number,
        "math::midhinge" => Kind::Number,
        "math::min" => Kind::Number,
        "math::mode" => Kind::Number,
        "math::nearestrank" => Kind::Number,
        "math::neg_inf" => Kind::Number,
        "math::percentile" => Kind::Number,
        "math::pi" => Kind::Number,
        "math::product" => Kind::Number,
        "math::rad2deg" => Kind::Number,
        "math::round" => Kind::Number,
        "math::sign" => Kind::Number,
        "math::sin" => Kind::Number,
        "math::tan" => Kind::Number,
        "math::tau" => Kind::Number,
        "math::spread" => Kind::Number,
        "math::sqrt" => Kind::Number,
        "math::stddev" => Kind::Number,
        "math::sum" => Kind::Number,
        "math::top" => Kind::Array(Box::new(Kind::Number), None),
        "math::trimean" => Kind::Number,
        "math::variance" => Kind::Number,

        // `time::` functions
        "time::day" => Kind::Number,
        "time::floor" => Kind::Number,
        "time::format" => Kind::String,
        "time::group" => Kind::Datetime,
        "time::hour" => Kind::Number,
        "time::max" => Kind::Datetime,
        "time::micros" => Kind::Number,
        "time::millis" => Kind::Number,
        "time::min" => Kind::Datetime,
        "time::minute" => Kind::Number,
        "time::month" => Kind::Number,
        "time::nano" => Kind::Number,
        "time::now" => Kind::Datetime,
        "time::round" => Kind::Datetime,
        "time::second" => Kind::Number,
        "time::timezone" => Kind::String,
        "time::unix" => Kind::Number,
        "time::wday" => Kind::Number,
        "time::week" => Kind::Number,
        "time::yday" => Kind::Number,
        "time::year" => Kind::Number,
        "time::from::micros" => Kind::Datetime,
        "time::from::millis" => Kind::Datetime,
        "time::from::nanos" => Kind::Datetime,
        "time::from::secs" => Kind::Datetime,
        "time::from::unix" => Kind::Datetime,

        // `duration::` functions
        "duration::days" => Kind::Number,
        "duration::hours" => Kind::Number,
        "duration::micros" => Kind::Number,
        "duration::millis" => Kind::Number,
        "duration::mins" => Kind::Number,
        "duration::nanos" => Kind::Number,
        "duration::secs" => Kind::Number,
        "duration::weeks" => Kind::Number,
        "duration::years" => Kind::Number,
        "duration::from::days" => Kind::Duration,
        "duration::from::hours" => Kind::Duration,
        "duration::from::micros" => Kind::Duration,
        "duration::from::millis" => Kind::Duration,
        "duration::from::mins" => Kind::Duration,
        "duration::from::nanos" => Kind::Duration,
        "duration::from::secs" => Kind::Duration,
        "duration::from::weeks" => Kind::Duration,

        // `crypto::` functions
        "crypto::md5" => Kind::String,
        "crypto::sha1" => Kind::String,
        "crypto::sha256" => Kind::String,
        "crypto::sha512" => Kind::String,
        "crypto::argon2::compare" => Kind::Bool,
        "crypto::argon2::generate" => Kind::String,
        "crypto::bcrypt::compare" => Kind::Bool,
        "crypto::bcrypt::generate" => Kind::String,
        "crypto::pbkdf2::compare" => Kind::Bool,
        "crypto::pbkdf2::generate" => Kind::String,
        "crypto::scrypt::compare" => Kind::Bool,
        "crypto::scrypt::generate" => Kind::String,

        // `meta::` functions
        "meta::id" => Kind::Any, // TODO: should this be a string?
        "meta::type" => Kind::String,

        // TODO: add more functions
        "array::len" => Kind::Number,
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
