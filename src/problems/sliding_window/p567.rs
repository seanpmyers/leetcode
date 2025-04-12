pub struct Solution;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        Self::alphabet_vec_sliding_window(s1, s2)
    }

    /// time: O(n) - only one loop through all n elements
    /// space: O(n) due to the hashmaps
    pub fn hashmap_sliding_window(s1: String, s2: String) -> bool {
        let s1: &[u8] = s1.as_bytes();
        let s2: &[u8] = s2.as_bytes();

        use std::collections::HashMap;
        let mut set1: HashMap<u8, i32> = HashMap::with_capacity(s1.len());
        s1.iter().for_each(|x| {
            set1.entry(*x).and_modify(|y| *y += 1).or_insert(1);
        });
        let mut set2: HashMap<u8, i32> = HashMap::with_capacity(s2.len());

        let mut l: usize = 0;
        for r in 0..s2.len() {
            set2.entry(s2[r]).and_modify(|x| *x += 1).or_insert(1);
            if (r - l + 1) == s1.len() {
                if set1.eq(&set2) {
                    return true;
                }
            }
            while (r - l + 1) >= s1.len() {
                set2.entry(s2[l]).and_modify(|x| *x -= 1);
                if let Some(v) = set2.get(&s2[l]) {
                    if v <= &0 {
                        set2.remove(&s2[l]);
                    }
                }

                l = l.saturating_add(1);
            }
        }

        false
    }

    /// time: O(n) - only one loop through all n elements
    /// space: O(1) since its O(2*26) -- two vectors of 26 character counts
    pub fn alphabet_vec_sliding_window(s1: String, s2: String) -> bool {
        let s1: &[u8] = s1.as_bytes();
        let s2: &[u8] = s2.as_bytes();

        let mut count1: [u32; 26] = [0; 26];
        const A: usize = b'a' as usize;
        for b in s1.iter() {
            count1[*b as usize - A] += 1;
        }
        let mut count2: [u32; 26] = [0; 26];

        let mut l: usize = 0;
        for r in 0..s2.len() {
            count2[s2[r] as usize - A] += 1;
            if (r - l + 1) == s1.len() {
                if count1.eq(&count2) {
                    return true;
                }
            }
            while (r - l + 1) >= s1.len() {
                count2[s2[l] as usize - A] -= 1;
                l = l.saturating_add(1);
            }
        }

        false
    }
}
