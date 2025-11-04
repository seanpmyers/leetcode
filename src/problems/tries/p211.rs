use std::{cell::RefCell, rc::Rc};

struct WordDictionary {
    pub root: Rc<RefCell<TrieNode>>,
}

pub struct TrieNode {
    pub terminal: bool,
    pub children: [Option<Rc<RefCell<TrieNode>>>; 26usize],
}

#[allow(dead_code)]
impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            root: Rc::new(RefCell::new(TrieNode {
                terminal: false,
                children: [const { None }; 26usize],
            })),
        }
    }

    pub fn get_index(input: u8) -> usize {
        (input - b'a') as usize
    }

    fn add_word(&mut self, word: String) {
        let mut current = self.root.clone();
        for byte in word.bytes() {
            let x = current.borrow().children[Self::get_index(byte)].clone();
            match x {
                Some(node) => current = node,
                None => {
                    let new = Rc::new(RefCell::new(TrieNode {
                        terminal: false,
                        children: [const { None }; 26usize],
                    }));

                    current.borrow_mut().children[Self::get_index(byte)] = Some(new.clone());

                    current = new;
                }
            };
        }

        current.borrow_mut().terminal = true;
    }

    fn search(&self, word: String) -> bool {
        Self::dfs(word.as_bytes(), self.root.clone())
    }

    fn dfs(bytes: &[u8], root: Rc<RefCell<TrieNode>>) -> bool {
        let Some(index) = bytes.first() else {
            return root.borrow().terminal;
        };

        match index {
            b'.' => {
                for child in root.borrow().children.iter().filter_map(|x| x.clone()) {
                    if Self::dfs(&bytes[1..], child) {
                        return true;
                    }
                }
                false
            }
            _ => {
                let Some(child) = root.borrow().children[Self::get_index(*index)].clone() else {
                    return false;
                };

                Self::dfs(&bytes[1..], child)
            }
        }
    }
}
