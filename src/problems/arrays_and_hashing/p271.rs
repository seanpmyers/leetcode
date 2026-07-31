#[allow(dead_code)]
pub mod second {

    struct Codec {}

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl Codec {
        fn new() -> Self {
            Self {}
        }

        fn encode(&self, strs: Vec<String>) -> String {
            let mut result: String = String::new();

            for text in strs.iter() {
                let bytes: &[u8] = text.as_bytes();
                result.push('#');
                let len: String = bytes.len().to_string();
                if bytes.len() < 100 {
                    result.push('0');
                }
                if bytes.len() < 10 {
                    result.push('0');
                }
                for x in len.chars() {
                    result.push(x);
                }
                for &b in bytes {
                    result.push(b as char);
                }
            }
            result
        }

        fn decode(&self, s: String) -> Vec<String> {
            let mut result: Vec<String> = Vec::with_capacity(200usize);

            let bytes: &[u8] = s.as_bytes();

            let mut current: usize = 1;

            while current < bytes.len() {
                let mut text_len: String = String::with_capacity(3);
                let end = current + 3;
                while current < end {
                    text_len.push(bytes[current] as char);
                    current += 1;
                }
                let mut word: String = String::new();
                let len: usize = text_len.parse::<usize>().unwrap();
                let end = current + len;
                while current < end {
                    word.push(bytes[current] as char);
                    current += 1;
                }
                result.push(word);
                current += 1;
            }

            result
        }
    }
}

pub mod first {
    pub struct Solution;
    impl Solution {
        pub fn encode(strs: Vec<String>) -> String {
            let mut result: String = String::new();
            for s in strs.into_iter() {
                result.push(s.len() as u8 as char);
                result.push('#');
                result.push_str(&s);
            }
            result
        }
        pub fn decode(s: String) -> Vec<String> {
            println!("{}", s);
            let mut result: Vec<String> = vec![];
            let chars: &[u8] = s.as_bytes();
            let mut i: usize = 0;
            while i < chars.len() {
                let mut x: usize = i;
                while chars[x] != '#' as u8 {
                    x += 1;
                }
                let count: usize = chars[x - 1] as usize;
                i = x + 1;
                x = i + count;
                result.push(str::from_utf8(&chars[i..x]).unwrap().to_string());
                i = x;
            }

            result
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT_1: [&str; 4] = ["neet", "code", "love", "you"];
    const INPUT_2: [&str; 4] = ["we", "say", ":", "yes"];
    use super::first::Solution;

    #[test]
    fn test_1() {
        let values: Vec<String> = INPUT_1.into_iter().map(|x| x.to_string()).collect();
        let encoded = Solution::encode(values.clone());
        assert_eq!(values, Solution::decode(encoded))
    }

    #[test]
    fn test_2() {
        let values: Vec<String> = INPUT_2.into_iter().map(|x| x.to_string()).collect();
        let encoded = Solution::encode(values.clone());
        assert_eq!(values, Solution::decode(encoded))
    }
}
