pub struct Solution {}
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            if digits[i] == 9 {
                digits[i] = 0;
                if i == 0 {
                    digits.insert(0, 1);
                    return digits;
                }
                continue;
            }
            digits[i] += 1;
            return digits;
        }
        digits
    }
}
