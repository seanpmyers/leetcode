pub mod first {
    pub struct Solution;
    use std::collections::{HashMap, VecDeque};
    impl Solution {
        pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
            let start: &[u8] = begin_word.as_bytes();
            let n: usize = word_list.len();
            let letters: usize = start.len();
            let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
            let mut map: HashMap<Vec<u8>, usize> = HashMap::new();

            for i in 0..n {
                map.insert(word_list[i].as_bytes().iter().cloned().collect(), i);
                for j in i + 1..n {
                    let diff = (0..letters)
                        .filter(|&k| {
                            let x = word_list[i].as_bytes();
                            let y = word_list[j].as_bytes();
                            x[k] != y[k]
                        })
                        .count();
                    if diff > 1 {
                        continue;
                    }
                    adj[i].push(j);
                    adj[j].push(i);
                }
            }

            let mut queue: VecDeque<usize> = VecDeque::with_capacity(n);
            let mut result: i32 = 1i32;
            let mut visited: Vec<bool> = vec![false; n];

            for i in 0..letters {
                for letter in b'a'..=b'z' {
                    if letter == start[i] {
                        continue;
                    }
                    let mut word: Vec<u8> = start.iter().cloned().collect();
                    word[i] = letter;
                    if let Some(w_idx) = map.get(&word) {
                        if !visited[*w_idx] {
                            visited[*w_idx] = true;
                            queue.push_back(*w_idx);
                        }
                    }
                }
            }

            while !queue.is_empty() {
                result += 1;
                let q_len: usize = queue.len();
                for _ in 0..q_len {
                    let Some(node) = queue.pop_front() else {
                        panic!("?")
                    };
                    if word_list[node] == end_word {
                        return result;
                    }
                    for &next in &adj[node] {
                        if visited[next] {
                            continue;
                        }
                        visited[next] = true;
                        queue.push_back(next);
                    }
                }
            }

            0i32
        }
    }
}
