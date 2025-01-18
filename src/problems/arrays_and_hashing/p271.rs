use core::str;

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

#[cfg(test)]
mod tests {
    const INPUT_1: [&str; 4] = ["neet", "code", "love", "you"];
    const INPUT_2: [&str; 4] = ["we", "say", ":", "yes"];
    use super::Solution;

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
