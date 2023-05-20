use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

pub fn serialize_skip_none<S>(
    map: &BTreeMap<String, Option<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let filtered_map = map
        .iter()
        .filter_map(|(key, value)| value.as_ref().map(|v| (key, v)))
        .collect::<BTreeMap<&String, &String>>();

    filtered_map.serialize(serializer)
}
