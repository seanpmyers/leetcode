pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let one = word1.as_bytes();
        let two = word2.as_bytes();
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut flip: bool = false;
        let mut result: String = String::new();
        while x < one.len() || y < two.len() {
            if x >= one.len() {
                while y < two.len() {
                    result.push(two[y] as char);
                    y += 1;
                }
            }
            if y >= two.len() {
                while x < one.len() {
                    result.push(one[x] as char);
                    x += 1;
                }
                return result;
            }

            match flip {
                true => {
                    result.push(two[y] as char);
                    y += 1;
                }
                false => {
                    result.push(one[x] as char);
                    x += 1;
                }
            }
            flip = !flip;
        }

        result
    }
}
