pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];

        Self::backtrack(&mut result, &nums, 0, vec![]);

        result
    }

    pub fn backtrack(
        result: &mut Vec<Vec<i32>>,
        input: &[i32],
        index: usize,
        mut current: Vec<i32>,
    ) {
        if index >= input.len() {
            if input.is_empty() {
                result.push(current);
            }
            return;
        }

        let mut x = input.to_vec();
        let value = x.remove(index);
        current.push(value);
        Self::backtrack(result, &x, 0, current.clone());
        current.pop();

        Self::backtrack(result, input, index.saturating_add(1), current);
    }
}
