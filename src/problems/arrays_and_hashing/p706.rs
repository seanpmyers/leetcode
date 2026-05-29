#[allow(dead_code)]
pub mod no_heap {
    struct MyHashMap {
        map: [i32; 1_000_001usize],
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl MyHashMap {
        fn new() -> Self {
            Self {
                map: [-1i32; 1_000_001usize],
            }
        }

        fn put(&mut self, key: i32, value: i32) {
            self.map[key as usize] = value;
        }

        fn get(&self, key: i32) -> i32 {
            self.map[key as usize]
        }

        fn remove(&mut self, key: i32) {
            self.map[key as usize] = -1i32;
        }
    }
}
