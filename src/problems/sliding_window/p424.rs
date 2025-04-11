pub struct Solution {}
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut max: i32 = 0i32;
        let bytes: &[u8] = s.as_bytes();
        let mut l: usize = 0;
        let mut max_f: i32 = 0;
        let mut map: HashMap<u8, i32> = HashMap::with_capacity(26usize);
        for r in 0..bytes.len() {
            let entry: &mut i32 = map.entry(bytes[r]).and_modify(|x| *x += 1).or_insert(1i32);
            max_f = max_f.max(*entry);
            while (r as i32 - l as i32 + 1i32) - max_f > k {
                map.entry(bytes[l]).and_modify(|x| *x -= 1);
                l = l.saturating_add(1);
            }
            max = max.max(r as i32 - l as i32 + 1i32);
        }

        max
    }
}
