use std::collections::HashMap;
struct TimeMap {
    pub map: HashMap<String, Vec<(i32, String)>>,
}

pub trait Solution {
    fn new() -> Self;
    fn set(&mut self, key: String, value: String, timestamp: i32);
    fn get(&self, key: String, timestamp: i32) -> String;
}

impl Solution for TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {}

    fn get(&self, key: String, timestamp: i32) -> String {
        String::new()
    }
}
