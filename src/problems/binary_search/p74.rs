pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix[0][0] > target || get_last_value(&matrix) < target {
            return false;
        }

        let nums: Vec<i32> = matrix.into_iter().flatten().collect();

        let mut start: usize = 0;
        let mut end: usize = nums.len();

        while start < end {
            let middle: usize = get_middle(start, end);
            match nums[middle].cmp(&target) {
                std::cmp::Ordering::Less => start = middle + 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => end = middle,
            }
        }

        false
    }
}

pub fn get_middle(start: usize, end: usize) -> usize {
    start + (end - start) / 2
}

pub fn get_last_value(matrix: &Vec<Vec<i32>>) -> i32 {
    let l = matrix[matrix.len() - 1].len() - 1;
    matrix[matrix.len() - 1][l]
}
