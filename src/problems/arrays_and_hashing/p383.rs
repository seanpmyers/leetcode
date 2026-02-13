pub mod hash_map {
    pub struct Solution;
    use std::collections::HashMap;
    impl Solution {
        pub fn can_construct(ransom_note: String, magazine: String) -> bool {
            if magazine.len() < ransom_note.len() {
                return false;
            }

            let rn: &[u8] = ransom_note.as_bytes();
            let mag: &[u8] = magazine.as_bytes();

            let mut map: HashMap<u8, usize> = HashMap::with_capacity(mag.len());

            for i in 0..mag.len() {
                map.entry(mag[i])
                    .and_modify(|count| *count += 1)
                    .or_insert(1usize);
            }

            for i in 0..rn.len() {
                let Some(count) = map.get_mut(&rn[i]) else {
                    return false;
                };

                if *count < 1 {
                    return false;
                }

                *count = count.checked_sub(1).unwrap();
            }

            true
        }
    }
}

pub mod array {
    pub struct Solution;
    impl Solution {
        pub fn can_construct(ransom_note: String, magazine: String) -> bool {
            if magazine.len() < ransom_note.len() {
                return false;
            }

            let rn: &[u8] = ransom_note.as_bytes();
            let mag: &[u8] = magazine.as_bytes();

            let mut count: [u32; 26usize] = [0u32; 26usize];

            for i in 0..mag.len() {
                let x: u8 = mag[i] - b'a';
                count[x as usize] += 1;
            }

            for i in 0..rn.len() {
                let x: u8 = rn[i] - b'a';
                if count[x as usize] == 0u32 {
                    return false;
                }
                count[x as usize] -= 1u32;
            }

            true
        }
    }
}
