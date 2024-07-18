use std::collections::HashMap;

use surrealdb::sql::Object;

use crate::QueryReturnType;

use super::{return_types::get_value_return_type, QueryState};

pub fn get_object_return_type(
    state: &mut QueryState,
    obj: &Object,
) -> Result<QueryReturnType, anyhow::Error> {
    let mut fields = HashMap::new();

    for (key, value) in obj.0.iter() {
        let return_type = get_value_return_type(value, &HashMap::new(), state)?;

        fields.insert(key.clone(), return_type);
    }

    return Ok(QueryReturnType::Object(fields));
}
