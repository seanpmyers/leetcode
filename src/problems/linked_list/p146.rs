pub mod unfortuante {
    use std::collections::HashMap;

    #[derive(Default, Debug, Clone)]
    pub struct Node {
        pub next: Option<usize>,
        pub previous: Option<usize>,
    }

    #[derive(Debug)]
    pub struct LRUCache {
        pub capacity: usize,
        pub list: Vec<(i32, Node)>,
        pub map: HashMap<i32, (i32, usize)>,
        pub tail: Option<usize>,
        pub head: Option<usize>,
    }
    #[allow(dead_code)]
    impl LRUCache {
        fn new(capacity: i32) -> Self {
            let capacity: usize = capacity as usize;
            Self {
                capacity,
                list: Vec::with_capacity(capacity),
                map: HashMap::with_capacity(capacity),
                tail: None,
                head: None,
            }
        }

        fn get(&mut self, key: i32) -> i32 {
            // println!("get {:?} key {}", self, key);
            if let Some((value, index)) = self.map.get_mut(&key) {
                let (_key, node) = self.list[*index].clone();

                if self.head == Some(*index) {
                    return *value;
                }

                self.list[*index].1.next = self.head;
                self.list[*index].1.previous = None;
                let old_head = self.head;
                self.head = Some(*index);
                match (node.previous, node.next) {
                    (None, None) => panic!("Should have been head... no previous or next"),
                    (None, Some(_next)) => {
                        panic!("No previous means it should have been the head...")
                    }
                    (Some(previous), None) => {
                        if Some(previous) == old_head {
                            self.list[previous].1.previous = Some(*index);
                        }
                        self.list[previous].1.next = None;
                        self.tail = Some(previous);
                    }
                    (Some(previous), Some(next)) => {
                        self.list[previous].1.next = Some(next);
                        self.list[next].1.previous = Some(previous);
                    }
                }

                return *value;
            }

            -1i32
        }

        fn put(&mut self, key: i32, value: i32) {
            // println!("put {:?} kv {} {}", self, key, value);

            if !self.capacity.eq(&self.list.len()) {
                match (self.head, self.tail) {
                    (None, None) => {
                        self.head = Some(0);
                        self.tail = Some(0);
                        self.list.push((
                            key,
                            Node {
                                next: None,
                                previous: None,
                            },
                        ));

                        self.map.insert(key, (value, 0));
                    }
                    (None, Some(_tail)) => panic!("No head but tail..."),
                    (Some(_head), None) => panic!("Head but no tail..."),
                    (Some(head), Some(_tail)) => {
                        self.list.push((
                            key,
                            Node {
                                next: None,
                                previous: Some(head),
                            },
                        ));

                        let index = self.list.len() - 1;
                        self.list[head].1.previous = Some(index);
                        self.head = Some(index);
                        self.map
                            .entry(key)
                            .and_modify(|x| *x = (value, index))
                            .or_insert((value, index));
                    }
                }
                return;
            }

            match (self.head, self.tail) {
                (None, None) => panic!("No head/tail but at capacity..."),
                (None, Some(_tail)) => panic!("No head but at capacity..."),
                (Some(_head), None) => panic!("No tail but at capacity..."),
                (Some(head), Some(tail)) => match self.map.get_mut(&key) {
                    Some(x) => {
                        x.0 = value;

                        if let Some(previous) = self.list[x.1].1.previous {
                            self.list[previous].1.next = self.list[x.1].1.next;
                        }
                        if tail == x.1 {
                            self.tail = self.list[x.1].1.previous;
                        }
                        self.list[x.1].1.previous = None;
                        self.list[x.1].1.next = self.head;
                        self.list[head].1.previous = Some(x.1);
                        self.head = Some(x.1);
                    }
                    None => {
                        let mut swop: (i32, Node) = (
                            key,
                            Node {
                                next: Some(head),
                                previous: None,
                            },
                        );

                        if let Some(tail_previous) = self.list[tail].1.previous {
                            self.list[tail_previous].1.next = None;
                            self.tail = Some(tail_previous);
                        }
                        self.list[head].1.previous = Some(tail);

                        self.head = Some(tail);
                        let to_delete = &self.list[tail];
                        self.map.remove(&to_delete.0);
                        self.map.insert(key, (value, tail));

                        std::mem::swap(&mut self.list[tail], &mut swop);
                    }
                },
            }
        }
    }
}

pub mod unfortuante_v2 {
    use std::collections::HashMap;

    #[derive(Default, Debug, Clone)]
    pub struct Node {
        pub next: Option<usize>,
        pub previous: Option<usize>,
    }

    #[derive(Debug)]
    pub struct LRUCache {
        pub capacity: usize,
        pub list: Vec<(i32, Node)>,
        pub map: HashMap<i32, (i32, usize)>,
        pub tail: Option<usize>,
        pub head: Option<usize>,
    }
    #[allow(dead_code)]
    impl LRUCache {
        fn new(capacity: i32) -> Self {
            let capacity: usize = capacity as usize;
            Self {
                capacity,
                list: Vec::with_capacity(capacity),
                map: HashMap::with_capacity(capacity),
                tail: None,
                head: None,
            }
        }

        fn get(&mut self, key: i32) -> i32 {
            println!("GET {:?} key {}", self, key);
            let map_entry: Option<(i32, usize)> = self.map.get(&key).cloned();
            if let Some((value, index)) = map_entry {
                Self::update_head(self, index);
                return value;
            }

            -1i32
        }

        fn put(&mut self, key: i32, value: i32) {
            println!("PUT {:?} kv {} {}", self, key, value);
            let node: Node = Node {
                next: None,
                previous: self.tail,
            };

            let map_entry: Option<(i32, usize)> = self.map.get(&key).cloned();
            let new_head_index: usize = match map_entry {
                Some((_map_value, map_index)) => {
                    self.map.entry(key).and_modify(|x| x.0 = value);
                    map_index
                }
                None => match self.list.len() >= self.capacity {
                    true => {
                        let tail = self
                            .tail
                            .expect("Tail missing when adding new element at capacity.");
                        let replaced_node: (i32, Node) = Self::repalce_element(self, key, tail);
                        self.map.remove_entry(&replaced_node.0);
                        self.map.insert(key, (value, tail));
                        tail
                    }
                    false => {
                        self.list.push((key, node));
                        let new_index = self.list.len() - 1;
                        self.map.insert(key, (value, new_index));
                        // match Self::update_tail(self) {
                        //     Some(old_tail) => {

                        //     }
                        //     None => (),
                        // }
                        new_index
                    }
                },
            };

            Self::update_head(self, new_head_index);
        }

        /// Replaces a node at the provided index with a new node with the provided key.
        /// References to previous and next nodes are preserved.
        fn repalce_element(&mut self, key: i32, replaced_index: usize) -> (i32, Node) {
            let replaced: &mut (i32, Node) = &mut self.list[replaced_index];
            let mut swop: (i32, Node) = (
                key,
                Node {
                    next: replaced.1.next,
                    previous: replaced.1.previous,
                },
            );
            std::mem::swap(replaced, &mut swop);

            swop
        }

        fn update_head(&mut self, new_head_index: usize) {
            if (self.head == Some(new_head_index) && self.tail == Some(new_head_index))
                || self.head == Some(new_head_index)
            {
                return;
            }
            if let Some(tail) = self.tail {
                if tail == new_head_index {
                    Self::update_tail(self);
                }
            }
            if let Some(head) = self.head {
                self.list[head].1.previous = Some(new_head_index);
                self.list[new_head_index].1.next = Some(head);
            }
            if self.tail.is_none() {
                self.tail = Some(new_head_index);
            }
            self.head = Some(new_head_index)
        }


    }
}


#[cfg(test)]
pub mod tests {}
