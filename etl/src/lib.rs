use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();
    // Iterate over each key-value pair in the input map
    for (&key, values) in h {
        // Iterate over each character in the values vector of characters
        for &value in values {
            // Insert a lowercase version of the current value as the key and the original key as the value
            result.insert(value.to_ascii_lowercase(), key);
        }
    }
    result
}
