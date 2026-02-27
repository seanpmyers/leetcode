pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: Vec::new(),
        }
    }
}

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let Some(n) = node else {
            return None;
        };
        let result: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
            val: n.borrow().val,
            neighbors: vec![],
        }));
        let mut map: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();
        let mut visited: HashSet<i32> = HashSet::new();
        map.entry(n.borrow().val).or_insert(result.clone());
        let mut stack: Vec<Rc<RefCell<Node>>> = vec![n.clone()];
        while let Some(x) = stack.pop() {
            let b = x.borrow_mut();
            if !visited.insert(b.val) {
                continue;
            };

            let parent: Rc<RefCell<Node>> = map
                .entry(b.val)
                .or_insert(Rc::new(RefCell::new(Node {
                    val: b.val,
                    neighbors: vec![],
                })))
                .clone();

            let mut pb = parent.borrow_mut();

            let mut neighbors: Vec<Rc<RefCell<Node>>> = Vec::new();
            for i in 0..b.neighbors.len() {
                let val = b.neighbors[i].borrow().val;
                let neighbor = b.neighbors[i].clone();
                let entry = map
                    .entry(val)
                    .or_insert(Rc::new(RefCell::new(Node {
                        val: val,
                        neighbors: vec![],
                    })))
                    .clone();
                stack.push(neighbor);
                neighbors.push(entry);
            }
            pb.neighbors.append(&mut neighbors);
        }

        Some(result)
    }
}
