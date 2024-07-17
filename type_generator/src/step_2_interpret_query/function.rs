// array::add
// array::all
// array::any
// array::append
// array::at
// array::boolean_and
// array::boolean_not
// array::boolean_or
// array::boolean_xor
// array::clump
// array::combine
// array::complement
// array::concat
// array::difference
// array::distinct
// array::filter_index
// array::find_index
// array::first
// array::flatten
// array::group
// array::insert
// array::intersect
// array::join
// array::last
// array::len
// array::logical_and
// array::logical_or
// array::logical_xor
// array::matches
// array::max
// array::min
// array::pop
// array::prepend
// array::push
// array::remove
// array::reverse
// array::shuffle
// array::slice
// array::sort
// array::transpose
// array::union
// array::sort::asc
// array::sort::desc
// bytes::len
// count::count
// crypto::md5
// crypto::sha1
// crypto::sha256
// crypto::sha512
// duration::days
// duration::hours
// duration::micros
// duration::millis
// duration::mins
// duration::nanos
// duration::secs
// duration::weeks
// duration::years
// duration::from::days
// duration::from::hours
// duration::from::micros
// duration::from::millis
// duration::from::mins
// duration::from::nanos
// duration::from::secs
// duration::from::weeks
// encoding::base64::decode
// encoding::base64::encode
// geo::area
// geo::bearing
// geo::centroid
// geo::distance
// geo::hash::decode
// geo::hash::encode

// not::not
// object::entries
// object::from_entries
// object::keys
// object::len
// object::values
// parse::email::host
// parse::email::user
// parse::url::domain
// parse::url::fragment
// parse::url::host
// parse::url::path
// parse::url::port
// parse::url::query
// parse::url::scheme
// rand::rand
// rand::bool
// rand::enum
// rand::float
// rand::guid
// rand::int
// rand::string
// rand::time
// rand::ulid
// rand::uuid::v4
// rand::uuid::v7
// rand::uuid
// session::ac
// session::db
// session::id
// session::ip
// session::ns
// session::origin
// session::rd
// session::token
// string::concat
// string::contains
// string::ends_with
// string::join
// string::len
// string::lowercase
// string::matches
// string::repeat
// string::replace
// string::reverse
// string::slice
// string::slug
// string::split
// string::starts_with
// string::trim
// string::uppercase
// string::words
// string::distance::hamming
// string::distance::levenshtein
// string::html::encode
// string::html::sanitize
// string::is::alphanum
// string::is::alpha
// string::is::ascii
// string::is::datetime
// string::is::domain
// string::is::email
// string::is::hexadecimal
// string::is::ip
// string::is::ipv4
// string::is::ipv6
// string::is::latitude
// string::is::longitude
// string::is::numeric
// string::is::semver
// string::is::url
// string::is::uuid
// string::similarity::fuzzy
// string::similarity::jaro
// string::similarity::smithwaterman
// string::semver::compare
// string::semver::major
// string::semver::minor
// string::semver::patch
// string::semver::inc::major
// string::semver::inc::minor
// string::semver::inc::patch
// string::semver::set::major
// string::semver::set::minor
// string::semver::set::patch
// time::ceil
// time::day
// time::floor
// time::format
// time::group
// time::hour
// time::max
// time::min
// time::minute
// time::month
// time::nano
// time::micros
// time::millis
// time::now
// time::round
// time::second
// time::timezone
// time::unix
// time::wday
// time::week
// time::yday
// time::year
// time::from::nanos
// time::from::micros
// time::from::millis
// time::from::secs
// time::from::unix
// type::bool
// type::datetime
// type::decimal
// type::duration
// type::float
// type::int
// type::number
// type::point
// type::string
// type::table
// type::thing
// type::range
// type::is::array
// type::is::bool
// type::is::bytes
// type::is::collection
// type::is::datetime
// type::is::decimal
// type::is::duration
// type::is::float
// type::is::geometry
// type::is::int
// type::is::line
// type::is::none
// type::is::null
// type::is::multiline
// type::is::multipoint
// type::is::multipolygon
// type::is::number
// type::is::object
// type::is::point
// type::is::polygon
// type::is::record
// type::is::string
// type::is::uuid
// vector::add
// vector::angle
// vector::cross
// vector::dot
// vector::divide
// vector::magnitude
// vector::multiply
// vector::normalize
// vector::project
// vector::subtract
// vector::distance::chebyshev
// vector::distance::euclidean
// vector::distance::hamming
// vector::distance::knn
// vector::distance::mahalanobis
// vector::distance::manhattan
// vector::distance::minkowski
// vector::similarity::cosine
// vector::similarity::jaccard
// vector::similarity::pearson
// vector::similarity::spearman

use surrealdb::sql::Function;

use crate::QueryReturnType;

pub fn get_function_return_type(func: &Function) -> Result<QueryReturnType, anyhow::Error> {
    match func {
        Function::Custom(..) => anyhow::bail!("Custom functions are not yet supported"),
        Function::Normal(name, ..) => normal_function_return_type(name),
        Function::Script(..) => anyhow::bail!("Script functions are not yet supported"),
        _ => anyhow::bail!("Unsupported function: {}", func),
    }
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
