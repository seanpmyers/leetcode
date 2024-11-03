pub struct Solution {}
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut start: usize = 0;
        let mut end: usize = numbers.len() - 1;
        while start != end {
            match target.cmp(&(numbers[start] + numbers[end])) {
                std::cmp::Ordering::Greater => start += 1,
                std::cmp::Ordering::Equal => return vec![(start + 1) as i32, (end + 1) as i32],
                std::cmp::Ordering::Less => end -= 1,
            }
        }
        vec![]
    }
}
