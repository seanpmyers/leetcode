pub mod stack {
    pub struct Solution;
    impl Solution {
        pub fn check_valid_string(s: String) -> bool {
            let s: &[u8] = s.as_bytes();

            let mut left: Vec<usize> = Vec::with_capacity(s.len());
            let mut star: Vec<usize> = Vec::with_capacity(s.len());

            for i in 0..s.len() {
                match s[i] {
                    b')' => {
                        if left.is_empty() && star.is_empty() {
                            return false;
                        }

                        if left.pop().is_some() {
                            continue;
                        }

                        star.pop();
                    }
                    b'(' => left.push(i),
                    b'*' => star.push(i),
                    _ => panic!("invalid state"),
                }
            }

            while let Some(l) = left.pop() {
                let Some(s) = star.pop() else {
                    return false;
                };
                if s < l {
                    return false;
                }
            }

            true
        }
    }
}
