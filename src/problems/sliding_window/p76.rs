pub struct Solution;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        pub fn solve(s: String, t: String) -> String {
            let s: &[u8] = s.as_bytes();
            let t: &[u8] = t.as_bytes();

            use std::collections::HashMap;
            let mut t_count: HashMap<u8, i32> = HashMap::with_capacity(t.len());
            t.iter().for_each(|byte| {
                t_count.entry(*byte).and_modify(|x| *x += 1).or_insert(1);
            });
            let mut window: HashMap<u8, i32> = HashMap::with_capacity(s.len());

            let mut l: usize = 0;
            let mut have: usize = 0;
            let mut result = (0, 0);
            let need: usize = t_count.len();
            let mut result_length: usize = usize::MAX;

            for r in 0..s.len() {
                window.entry(s[r]).and_modify(|x| *x += 1).or_insert(1);
                if window.get(&s[r]).eq(&t_count.get(&s[r])) {
                    have = have.saturating_add(1);
                }

                while have == need {
                    if (r - l + 1) < result_length {
                        result_length = r - l + 1;
                        result.0 = l;
                        result.1 = r;
                    }

                    window.entry(s[l]).and_modify(|x| *x -= 1);
                    if let Some(count) = t_count.get(&s[l]) {
                        if let Some(w) = window.get(&s[l]) {
                            if w < count {
                                have = have.saturating_sub(1);
                            }
                        }
                    }
                    l = l.saturating_add(1);
                }
            }

            match result_length.eq(&usize::MAX) {
                true => String::new(),
                false => String::from_utf8(s[result.0..result.1 + 1].to_vec()).unwrap(),
            }
        }
        match s.len().cmp(&t.len()) {
            std::cmp::Ordering::Less => String::new(),
            std::cmp::Ordering::Equal => solve(s, t),
            std::cmp::Ordering::Greater => solve(s, t),
        }
    }
}
