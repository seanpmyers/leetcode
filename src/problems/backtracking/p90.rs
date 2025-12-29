pub struct Solution;
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(10);
        result.push(vec![]);

        let mut previous_index = 0;

        for i in 0..nums.len() {
            let index = match i > 0 && nums[i] == nums[i - 1] {
                true => previous_index,
                false => 0,
            };
            previous_index = result.len();
            for j in index..previous_index {
                let mut x = result[j].clone();
                x.push(nums[i]);
                result.push(x);
            }
        }

        result
    }
}
