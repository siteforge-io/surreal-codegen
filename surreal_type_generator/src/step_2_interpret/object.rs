use std::collections::BTreeMap;

use surrealdb::sql::Object;

use crate::ValueType;

use super::{return_types::get_value_return_type, QueryState};

pub fn get_object_return_type(
    state: &mut QueryState,
    obj: &Object,
) -> Result<ValueType, anyhow::Error> {
    let mut fields = BTreeMap::new();

    for (key, value) in obj.0.iter() {
        let return_type = get_value_return_type(value, &BTreeMap::new(), state)?;

        fields.insert(key.clone(), return_type);
    }

    return Ok(ValueType::Object(fields));
}
