pub mod hashset {
    pub struct Solution;
    use std::cmp::Ordering;
    use std::collections::HashSet;
    impl Solution {
        pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::with_capacity(nums.len() / 2);
            nums.sort();
            let mut set: HashSet<(i32, i32, i32, i32)> = HashSet::with_capacity(nums.len());
            for i in 0..nums.len() {
                for j in i + 1..nums.len() {
                    let mut l: usize = j + 1;
                    let mut r: usize = nums.len() - 1;
                    let w: i32 = nums[i];
                    let x: i32 = nums[j];

                    while l < r && l < nums.len() {
                        let y: i32 = nums[l];
                        let z: i32 = nums[r];
                        let sum: i32 = w.saturating_add(x).saturating_add(y).saturating_add(z);
                        match sum.cmp(&target) {
                            Ordering::Equal => {
                                if set.insert((w, x, y, z)) {
                                    result.push(vec![w, x, y, z]);
                                }
                                l += 1;
                            }
                            Ordering::Less => l += 1,
                            Ordering::Greater => r -= 1,
                        };
                    }
                }
            }

            result
        }
    }
}
