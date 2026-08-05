pub mod two_pointer {
    pub struct Solution;
    use std::cmp::Ordering;
    impl Solution {
        pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            nums.sort();
            let mut result: Vec<Vec<i32>> = Vec::with_capacity(nums.len());
            for i in 0..nums.len() {
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }
                let x: i32 = nums[i];
                let mut l: usize = i + 1;
                let mut r: usize = nums.len() - 1;

                while l < r {
                    let y = nums[l];
                    let z = nums[r];
                    match (x + y + z).cmp(&0) {
                        Ordering::Equal => {
                            if l != r && l != i {
                                result.push(vec![x, y, z]);
                            }
                            while l < r && y == nums[l] {
                                l += 1;
                            }
                        }
                        Ordering::Less => {
                            l += 1;
                        }
                        Ordering::Greater => {
                            r -= 1;
                        }
                    }
                }
            }

            result
        }
    }
}
