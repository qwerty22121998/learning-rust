// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map: HashMap<String, i32> = HashMap::new();
    magazine.iter().for_each(|x| {
        *map.entry(x.to_string()).or_insert(0) += 1;
    });
    let mut result = true;
    note.iter().for_each(|x| {
        if map.get(&x.to_string()).unwrap_or(&0) <= &0i32 {
            result = false;
            return;
        }
        map.entry(x.to_string()).and_modify(|e| *e -= 1);
    });
    result
}
