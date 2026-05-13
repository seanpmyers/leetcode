pub mod swapping {
    pub struct Solution;
    impl Solution {
        pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            if nums.is_empty() {
                return vec![];
            }
            let mut result: Vec<Vec<i32>> = vec![];
            fn dfs(i: usize, nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
                if i == nums.len() {
                    result.push(nums.clone());
                    return;
                }

                for j in i..nums.len() {
                    nums.swap(i, j);
                    dfs(i + 1, nums, result);
                    nums.swap(i, j);
                }
            }
            dfs(0usize, &mut nums, &mut result);
            result
        }
    }
}
pub mod iterative {
    pub struct Solution;
    impl Solution {
        pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = vec![vec![]];

            for &n in nums.iter() {
                let mut perms: Vec<Vec<i32>> = vec![];
                for p in &result {
                    for i in 0..p.len() + 1 {
                        let mut copy = p.clone();
                        copy.insert(i, n);
                        perms.push(copy);
                    }
                }
                result = perms;
            }

            result
        }
    }
}
pub mod backtracking {
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
}

pub mod backtrack_2 {
    pub struct Solution;
    impl Solution {
        pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
            Self::recurse(0usize, &nums)
        }

        pub fn recurse(i: usize, nums: &Vec<i32>) -> Vec<Vec<i32>> {
            if i >= nums.len() {
                return vec![vec![]];
            }

            let mut result: Vec<Vec<i32>> = vec![];
            let perms: Vec<Vec<i32>> = Self::recurse(i + 1, nums);

            for p in perms.into_iter() {
                for j in 0..p.len() + 1 {
                    let mut copy = p.clone();
                    copy.insert(j, nums[i]);
                    result.push(copy);
                }
            }

            result
        }
    }
}
