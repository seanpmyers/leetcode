pub mod recursive {
    pub struct Solution;
    impl Solution {
        pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            nums.sort();
            let mut result: Vec<Vec<i32>> = vec![];

            Self::backtrack(&mut result, &nums, 0usize, &mut vec![]);

            result
        }

        pub fn backtrack(
            result: &mut Vec<Vec<i32>>,
            nums: &Vec<i32>,
            i: usize,
            current: &mut Vec<i32>,
        ) {
            if i >= nums.len() {
                result.push(current.clone());
                return;
            }

            current.push(nums[i]);
            Self::backtrack(result, nums, i + 1, current);
            current.pop();

            if current.is_empty()
                || (current.len() > 0 && i > 0 && nums[i] != current[current.len() - 1])
            {
                Self::backtrack(result, nums, i + 1, current);
            }
        }
    }
}
pub mod iterative {
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
}
