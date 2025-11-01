use std::{cell::RefCell, rc::Rc};

#[allow(dead_code)]
struct Trie {
    list: [Option<Rc<RefCell<TrieNode>>>; 26],
}

#[allow(dead_code)]
struct TrieNode {
    content: u8,
    end: bool,
    list: [Option<Rc<RefCell<TrieNode>>>; 26],
}

#[allow(dead_code)]
impl Trie {
    fn new() -> Self {
        Trie {
            list: [const { None }; 26usize],
        }
    }

    fn get_index(input: u8) -> usize {
        (input - b'a') as usize
    }

    fn insert(&mut self, word: String) {
        let word: &[u8] = word.as_bytes();
        let start_index: usize = Self::get_index(word[0]);

        if let None = &self.list[start_index] {
            self.list[start_index] = Some(Rc::new(RefCell::new(TrieNode {
                content: word[0],
                end: word.len() == 1,
                list: [const { None }; 26usize],
            })));
        };

        if word.len() == 1 {
            return;
        }

        let mut current: Rc<RefCell<TrieNode>> = self.list[start_index].clone().unwrap();

        for (index, byte) in word.iter().enumerate().skip(1usize) {
            let next: &Option<Rc<RefCell<TrieNode>>> =
                &current.borrow().list[Self::get_index(*byte)].clone();
            match next {
                Some(n) => {
                    if index == (word.len() - 1) {
                        n.borrow_mut().end = true;
                    }
                    current = n.clone();
                }
                None => {
                    let node = Rc::new(RefCell::new(TrieNode {
                        content: *byte,
                        end: index == (word.len() - 1),
                        list: [const { None }; 26usize],
                    }));
                    current.borrow_mut().list[Self::get_index(*byte)] = Some(node.clone());
                    current = node.clone();
                }
            }
        }
    }

    fn search(&self, word: String) -> bool {
        let word: &[u8] = word.as_bytes();
        let current: Option<Rc<RefCell<TrieNode>>> = self.list[Self::get_index(word[0])].clone();

        let Some(mut node) = current else {
            return false;
        };

        for (_index, byte) in word.iter().skip(1).enumerate() {
            let maybe_next: Option<Rc<RefCell<TrieNode>>> =
                node.borrow().list[Self::get_index(*byte)].clone();
            match maybe_next {
                Some(next) => node = next.clone(),
                None => {
                    return false;
                }
            };
        }

        let end: bool = node.borrow().end;

        end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let prefix: &[u8] = prefix.as_bytes();

        let Some(mut current) = self.list[Self::get_index(prefix[0])].clone() else {
            return false;
        };

        for byte in prefix.iter().skip(1) {
            let maybe_next: Option<Rc<RefCell<TrieNode>>> =
                current.borrow().list[Self::get_index(*byte)].clone();
            match maybe_next {
                Some(next) => current = next.clone(),
                None => {
                    return false;
                }
            }
        }

        true
    }
}
