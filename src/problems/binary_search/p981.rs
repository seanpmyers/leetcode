#[allow(dead_code)]
pub mod binary_search {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    struct TimeMap {
        pub map: HashMap<String, Vec<(String, i32)>>,
    }

    impl TimeMap {
        fn new() -> Self {
            Self {
                map: HashMap::new(),
            }
        }

        fn set(&mut self, key: String, value: String, timestamp: i32) {
            self.map.entry(key).or_default().push((value, timestamp));
        }

        fn get(&self, key: String, timestamp: i32) -> String {
            let Some(list) = self.map.get(&key) else {
                return String::new();
            };

            let mut l: usize = 0;
            let mut r: usize = list.len();

            if timestamp > list[r - 1].1 {
                return list[r - 1].0.clone();
            }
            if timestamp < list[0].1 {
                return String::new();
            }

            while l < r {
                let mut middle: usize = r.midpoint(l);
                match list[middle].1.cmp(&timestamp) {
                    Ordering::Equal => {
                        while middle + 1 < list.len() && list[middle + 1].1 == timestamp {
                            middle += 1;
                        }
                        return list[middle].0.clone();
                    }
                    Ordering::Less => l = middle + 1,
                    Ordering::Greater => r = middle,
                };
            }

            list[l - 1].0.clone()
        }
    }
}
pub mod first {
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
}
