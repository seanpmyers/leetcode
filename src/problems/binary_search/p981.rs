use std::collections::HashMap;
pub struct TimeMap {
    pub map: HashMap<String, Vec<(i32, String)>>,
}

pub trait Solution {
    fn new() -> Self;
    fn set(&mut self, key: String, value: String, timestamp: i32);
    fn get(&self, key: String, timestamp: i32) -> String;
}

impl TimeMap {
    pub fn binary_search(&self, timestamp: &i32, list: &Vec<(i32, String)>) -> Option<String> {
        let middle = |l: usize, r: usize| -> usize { (l + r) / 2 };

        let mut result: Option<String> = None;

        let mut l: usize = 0;
        let mut r: usize = list.len() - 1;

        if &list[l].0 > timestamp {
            return result;
        }

        if &list[l].0 > timestamp {
            return result;
        }

        if list.len() == 2 {
            if &list[r].0 <= timestamp {
                return Some(list[r].1.clone());
            }
            return Some(list[l].1.clone());
        }

        while l <= r {
            let middle: usize = middle(l, r);

            match list[middle].0.cmp(timestamp) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    result = Some(list[middle].1.clone());
                    l = middle.saturating_add(1);
                }
                std::cmp::Ordering::Greater => r = middle.saturating_sub(1),
            }
        }

        result
    }
}

impl Solution for TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map
            .entry(key)
            .and_modify(|map| map.push((timestamp, value.clone())))
            .or_insert(vec![(timestamp, value)]);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if self.map.is_empty() {
            return String::new();
        }

        let values: Option<&Vec<(i32, String)>> = self.map.get(&key);

        let Some(values): Option<&Vec<(i32, String)>> = values else {
            return String::new();
        };

        self.binary_search(&timestamp, values)
            .unwrap_or(String::new())
    }
}
