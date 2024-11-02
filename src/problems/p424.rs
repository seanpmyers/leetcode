pub struct Solution {}
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        if k == s.len() as i32 {
            return k;
        }
        let mut result: i32 = 1;
        let chars: Vec<char> = s.chars().collect();
        let mut s: usize = 0;
        let mut e: usize = 0;
        let mut current: i32 = k;
        while e < chars.len() {
            if chars[e] != chars[s] {
                if current > 0 {
                    current -= 1;
                    e += 1;
                    continue;
                }
                current = k;
                result = result.max(chars[s..e].len() as i32);
                println!("{} {} {:?}", s, e, chars[s..e].to_vec());
                s += 1;
                e = s + 1;
                continue;
            }
            e += 1;
        }

        result.max(chars[s..e].len() as i32)
    }
}
