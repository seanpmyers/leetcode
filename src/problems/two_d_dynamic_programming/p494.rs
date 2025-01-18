pub struct Solution {}
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut result: i32 = 0_i32;
        let i: usize = 0_usize;
        let sum: i32 = 0_i32;
        backtrack(i, sum, &mut result, &nums, target);
        result
    }
}

pub fn backtrack(i: usize, sum: i32, result: &mut i32, nums: &Vec<i32>, target: i32) {
    println!("{sum} {i} {result}");
    if i == nums.len() && sum == target {
        println!("here");
        *result += 1i32;
        return;
    }
    if i >= nums.len() {
        return;
    }
    backtrack(i + 1, sum + nums[i], result, nums, target);
    backtrack(i + 1, sum - nums[i], result, nums, target);
}

#[cfg(test)]
mod tests {
    use crate::problems::two_d_dynamic_programming::p494::Solution;

    const NUMS_BASIC: [i32; 5] = [1i32, 1i32, 1i32, 1i32, 1i32];

    #[test]
    fn basic_case() {
        let result = Solution::find_target_sum_ways(NUMS_BASIC.to_vec(), 3i32);
        assert_eq!(result, 5)
    }
}
