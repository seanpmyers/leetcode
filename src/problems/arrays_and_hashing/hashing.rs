#[derive(Debug, Clone)]
pub struct Entry {
    key: String,
    value: String,
}

#[derive(Debug)]
pub struct HashMap {
    size: usize,
    capacity: usize,
    map: Vec<Option<Entry>>,
}

impl HashMap {
    pub fn new() -> Self {
        Self {
            size: 0,
            capacity: 3,
            map: Vec::with_capacity(3),
        }
    }

    fn grow(&mut self, new_capacity: usize) {
        let mut result: Vec<Option<Entry>> = vec![None; new_capacity];
        self.capacity = new_capacity;
        for i in 0..self.map.len() {
            let Some(entry) = self.map[i].take() else {
                continue;
            };
            let index: usize = self.hash(&entry.key);
            result[index] = Some(entry);
        }
        self.map = result;
    }

    fn hash(&self, input: &str) -> usize {
        input.chars().map(|c| c as usize).sum::<usize>() % self.capacity
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let index: usize = self.hash(key);
        self.map[index].clone().map(|x| x.value.clone())
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.size += 1;
        if self.size >= (self.capacity / 2) {
            self.grow((self.capacity * 2) + 1);
        }
        let index: usize = self.hash(&key);
        self.map[index] = Some(Entry { key, value });
    }
}

#[cfg(test)]
pub mod test {

    #[test]
    pub fn hash_test() {
        let mut map = crate::problems::arrays_and_hashing::hashing::HashMap::new();
        map.insert("Test".to_string(), "Map".to_string());
        map.insert("Joe".to_string(), "Bins".to_string());
        map.insert("Frog".to_string(), "Jimmy".to_string());

        assert_eq!(map.get(&"Test"), Some("Map".to_string()));
        assert_eq!(map.get(&"Joe"), Some("Bins".to_string()));
        assert_eq!(map.get(&"Frog"), Some("Jimmy".to_string()));
    }
}
