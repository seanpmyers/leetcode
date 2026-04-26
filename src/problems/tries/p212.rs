pub mod first {
    use std::collections::HashSet;
    #[derive(Debug)]
    pub struct Trie {
        pub data: Vec<Edge>,
    }

    #[derive(Debug)]
    pub struct Edge {
        pub next: [Option<usize>; 26usize],
        pub word: Option<String>,
    }

    impl Edge {
        pub fn new() -> Self {
            Self {
                next: [const { None }; 26usize],
                word: None,
            }
        }
    }

    impl Trie {
        pub fn new() -> Self {
            Self {
                data: vec![Edge::new()],
            }
        }

        pub fn byte_index(byte: u8) -> usize {
            (byte - b'a') as usize
        }

        fn insert(&mut self, word: &String) {
            let bytes: &[u8] = word.as_bytes();
            let mut current: usize = 0usize;

            for i in 0..bytes.len() {
                let b_index: usize = Self::byte_index(bytes[i]);

                let Some(next) = self.data[current].next[b_index] else {
                    let next: usize = self.data.len();
                    self.data[current].next[b_index] = Some(next);
                    current = next;
                    self.data.push(Edge::new());
                    continue;
                };

                current = next;
            }

            self.data[current].word = Some(word.clone());
        }

        pub fn find(&self, letter: u8, index: usize) -> Option<usize> {
            self.data[index].next[Self::byte_index(letter)]
        }
    }

    pub struct Solution;
    impl Solution {
        pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
            let mut trie: Trie = Trie::new();

            for word in &words {
                trie.insert(word);
            }

            let mut result: HashSet<String> = HashSet::new();

            for r in 0..board.len() {
                for c in 0..board[0].len() {
                    Self::dfs((r, c), 0usize, &trie, &mut board, &mut result);
                }
            }

            result.into_iter().collect()
        }

        pub fn dfs(
            (r, c): (usize, usize),
            mut index: usize,
            trie: &Trie,
            board: &mut Vec<Vec<char>>,
            result: &mut HashSet<String>,
        ) {
            if board[r][c] == '#' {
                return;
            }

            let Some(next) = trie.find(board[r][c] as u8, index) else {
                return;
            };

            index = next;
            let rows: usize = board.len();
            let columns: usize = board[0].len();

            if let Some(word) = &trie.data[index].word {
                result.insert(word.clone());
            }
            let letter = board[r][c];
            board[r][c] = '#';
            let directions: [(isize, isize); 4usize] = [
                (-1, 0), // up
                (1, 0),  // down
                (0, -1), // left
                (0, 1),  // right
            ];

            for &d in &directions {
                let (Some(row), Some(column)) =
                    (r.checked_add_signed(d.0), c.checked_add_signed(d.1))
                else {
                    continue;
                };
                if row >= rows || column >= columns {
                    continue;
                }
                Self::dfs((row, column), index, trie, board, result);
            }
            board[r][c] = letter;
        }
    }
}
