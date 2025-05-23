// src/utils.rs
use serde_json::Value;

/// Extrahuje text vhodný pre AI analýzu zo štruktúry JSON.
pub fn extract_text_for_ai(data: &Value) -> Option<String> {
    match data {
        Value::String(s) => Some(s.clone()),
        Value::Object(map) => map.values().filter_map(extract_text_for_ai).collect::<Vec<_>>().join(" ").into(),
        Value::Array(arr) => arr.iter().filter_map(extract_text_for_ai).collect::<Vec<_>>().join(" ").into(),
        _ => None,
    }
}
