use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type LinkedListNode = Rc<RefCell<Node>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub key: i32,
    pub value: i32,
    pub previous: Option<LinkedListNode>,
    pub next: Option<LinkedListNode>,
}

#[derive(Debug, Clone)]
pub struct LRUCache {
    pub capacity: usize,
    pub map: HashMap<i32, LinkedListNode>,
    pub head: Option<LinkedListNode>,
    pub tail: Option<LinkedListNode>,
}

impl Node {
    pub fn new(key: i32, value: i32) -> LinkedListNode {
        Rc::new(RefCell::new(Node {
            key,
            value,
            previous: None,
            next: None,
        }))
    }
}

#[allow(dead_code)]
impl LRUCache {
    const NOT_FOUND: i32 = -1i32;

    fn new(capacity: i32) -> Self {
        let capacity: usize = capacity as usize;
        Self {
            capacity,
            map: HashMap::with_capacity(capacity),
            head: None,
            tail: None,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(entry) = self.map.get(&key).cloned() {
            let value: i32 = entry.borrow().value;
            if !self.head.eq(&Some(entry.clone())) {
                self.replace_head(entry);
            }
            return value;
        }
        Self::NOT_FOUND
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.is_empty() {
            let node = Node::new(key, value);
            self.tail = Some(node.clone());
            self.map.insert(key, node.clone());
            self.head = Some(node);
            return;
        }

        if !self.map.contains_key(&key) && self.map.len() >= self.capacity {
            while self.map.len() >= self.capacity {
                Self::remove_tail(self);
            }
        }

        match self.map.get(&key) {
            Some(node) => {
                node.borrow_mut().value = value;
                Self::replace_head(self, node.clone());
            }
            None => {
                let node: LinkedListNode = Node::new(key, value);
                self.map.insert(key, node.clone());
                Self::replace_head(self, node);
            }
        }
    }

    fn remove_tail(&mut self) {
        if self.head.eq(&self.tail) {
            let key: i32 = self.tail.take().unwrap().borrow().key;
            self.head = None;
            self.tail = None;
            self.map.remove(&key);
            return;
        }

        let Some(tail) = self.tail.take() else {
            return;
        };

        self.map.remove(&tail.borrow().key);

        if tail.borrow().next.is_some() {
            panic!("Invalid state: tail has a next");
        }

        let Some(previous) = tail.borrow_mut().previous.take() else {
            self.tail = None;
            return;
        };
        previous.borrow_mut().next = None;
        self.tail = Some(previous);
    }

    fn replace_head(&mut self, node: LinkedListNode) {
        // point the previous to next
        // take head and point node to it
        if self.head.eq(&Some(node.clone())) {
            return;
        }

        let is_tail: bool = self.tail.eq(&Some(node.clone()));

        let next = node.borrow_mut().next.take();
        if let Some(previous) = node.borrow_mut().previous.take() {
            previous.borrow_mut().next = next.clone();
            if let Some(next) = next {
                next.borrow_mut().previous = Some(previous.clone());
            };
            if is_tail {
                self.tail = Some(previous);
            }
        }
        match self.head.take() {
            Some(head) => {
                head.borrow_mut().previous = Some(node.clone());
                node.borrow_mut().next = Some(head);
                self.head = Some(node);
            }
            None => {
                self.map.insert(node.borrow().key, node.clone());
                self.tail = Some(node.clone());
                self.head = Some(node);
            }
        }
    }
}
