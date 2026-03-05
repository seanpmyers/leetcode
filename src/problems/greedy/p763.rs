pub mod first_attempt {
    pub struct Solution;
    use std::collections::HashMap;
    impl Solution {
        pub fn partition_labels(s: String) -> Vec<i32> {
            let mut result: Vec<i32> = Vec::with_capacity(s.len());
            let mut map: HashMap<u8, Vec<usize>> = HashMap::with_capacity(s.len());
            let s: &[u8] = s.as_bytes();

            for i in 0..s.len() {
                map.entry(s[i])
                    .and_modify(|list| list.push(i))
                    .or_insert(vec![i]);
            }

            let mut i: usize = 0;

            while i < s.len() {
                let Some(list) = map.get(&s[i]) else {
                    panic!("?");
                };
                let first: usize = list[0];
                let mut last: usize = list[list.len() - 1];
                if first == last && i <= first {
                    result.push(1i32);
                    i += 1;
                    continue;
                }
                let mut x = i + 1;
                while x < last {
                    let Some(list) = map.get(&s[x]) else {
                        panic!("?");
                    };
                    last = list[list.len() - 1].max(last);
                    x += 1;
                }
                result.push((last - i) as i32 + 1);
                i = last + 1;
            }

            result
        }
    }
}
