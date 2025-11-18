use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

pub fn get_string_from_item(value: &str, item: &HashMap<String, AttributeValue>) -> String {
    item.get(value)
        .and_then(|v| v.as_s().ok())
        .cloned()
        .unwrap_or_default()
}
pub fn get_string_from_item_to_option(
    value: &str,
    items: &HashMap<String, AttributeValue>,
) -> Option<String> {
    items
        .get(value)
        .and_then(|v| v.as_s().ok())
        .map(|s| s.to_string())
}

pub fn get_bool_from_item_to_option(
    value: &str,
    items: &HashMap<String, AttributeValue>,
) -> Option<bool> {
    items.get(value).and_then(|v| v.as_bool().ok()).copied()
}
