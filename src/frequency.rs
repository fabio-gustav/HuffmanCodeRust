//contains a function that counts the number of times a char is in the passed in string slice
//and returns a hashmap of the corresponding frequencies

//imports the hashmap type
use std::collections::HashMap;

pub fn frequency(item: &str) -> HashMap<char, i32> {
    //make a new hashmap
    let mut map = HashMap::new();

    //for each char in the string, add it to the hashmap if not already and increase the count of the specific char by one
    for character in item.chars() {
        let count = map.entry(character).or_insert(0);
        *count += 1;
    }

    //returns the completed hashmap
    return map;
}