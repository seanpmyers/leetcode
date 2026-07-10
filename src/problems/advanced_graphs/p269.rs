pub mod topological {
    pub struct Solution;
    impl Solution {
        pub fn alien_order(words: Vec<String>) -> String {
            use std::collections::VecDeque;
            let mut letters: [bool; 26usize] = [false; 26usize];
            let mut indegree: [usize; 26usize] = [0usize; 26usize];
            let mut adj: Vec<Vec<usize>> = vec![vec![]; 26usize];
            for word in words.iter() {
                let word = word.as_bytes();
                for &c in word {
                    let i: usize = (c - b'a') as usize;
                    letters[i] = true;
                }
            }

            for i in 0..words.len().saturating_sub(1) {
                let x: &[u8] = words[i].as_bytes();
                let y: &[u8] = words[i + 1].as_bytes();
                let min: usize = x.len().min(y.len());

                if x.len() > y.len() && x[..min] == y[..min] {
                    return String::new();
                }

                for j in 0..min {
                    if x[j] == y[j] {
                        continue;
                    }
                    let xl: usize = (x[j] - b'a') as usize;
                    let yl: usize = (y[j] - b'a') as usize;
                    adj[xl].push(yl);
                    indegree[yl] += 1;
                    break;
                }
            }

            let mut queue: VecDeque<usize> = VecDeque::new();

            for i in 0..26 {
                if !letters[i] || indegree[i] != 0 {
                    continue;
                }
                queue.push_back(i);
            }

            let mut result: String = String::new();

            while let Some(done) = queue.pop_front() {
                result.push((done as u8 + b'a') as char);
                for &next in &adj[done] {
                    indegree[next] = indegree[next].saturating_sub(1);
                    if indegree[next] == 0 {
                        queue.push_back(next);
                    }
                }
            }

            for i in 0..26 {
                if !letters[i] {
                    continue;
                }
                if indegree[i] != 0 {
                    return String::new();
                }
            }

            result
        }
    }
}
