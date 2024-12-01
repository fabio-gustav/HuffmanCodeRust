use std::collections::HashMap;

pub fn frequency(item: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();

    for character in item.chars() {
        let count = map.entry(character).or_insert(0);
        *count += 1;
    }

    return map;
}