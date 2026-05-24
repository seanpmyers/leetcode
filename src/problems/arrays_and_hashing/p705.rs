#[allow(dead_code)]
pub mod all_stack {
    struct MyHashSet {
        set: [bool; 1_000_001usize],
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl MyHashSet {
        fn new() -> Self {
            Self {
                set: [false; 1_000_001usize],
            }
        }

        fn add(&mut self, key: i32) {
            self.set[key as usize] = true;
        }

        fn remove(&mut self, key: i32) {
            self.set[key as usize] = false;
        }

        fn contains(&self, key: i32) -> bool {
            self.set[key as usize]
        }
    }
}
