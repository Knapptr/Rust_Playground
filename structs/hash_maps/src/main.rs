use std::collections::hash_map::OccupiedEntry;
#[allow(dead_code, unused)]
use std::collections::HashMap;

fn main() {
    let test_set = vec![2u8, 2u8, 8u8, 9u8, 3u8];
    let created_set = create_set(test_set);
    println!("{:#?}", created_set);
}

fn create_set(vec: Vec<u8>) -> Vec<u8> {
    let mut map = HashMap::new();
    let mut set = Vec::new();
    for value in vec {
        match map.get(&value) {
            Some(val) => (),
            _ => {
                map.insert(value, true);
                set.push(value);
            }
        }
    }

    set
}

fn occupied<k, v>(mut map: HashMap<k, v>) {
    let result = map.ent
}
