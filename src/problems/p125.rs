pub struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s
            .chars()
            .filter(|c| is_ascii_alpha_numeric(*c as usize))
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<char>>();
        if s.is_empty() || s.len() == 1 {
            return true;
        }

        let end: usize = s.len() - 1;
        let middle: usize = s.len() / 2;
        for i in 0..s.len() {
            if i == middle {
                break;
            }
            if s[i] != s[end - i] {
                return false;
            }
        }
        true
    }
}

// there is a built in method but here is an example of how you would wirte a method
pub fn is_ascii_alpha_numeric(input: usize) -> bool {
    let a: usize = 'a' as usize;
    let z: usize = 'z' as usize;
    let big_a: usize = 'A' as usize;
    let big_z: usize = 'Z' as usize;
    let zero: usize = '0' as usize;
    let nine: usize = '9' as usize;
    input >= a && input <= z || input >= big_a && input <= big_z || input >= zero && input <= nine
}
