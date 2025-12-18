pub mod greedy_clean {
    pub struct Solution;
    impl Solution {
        pub fn can_jump(nums: Vec<i32>) -> bool {
            if nums.len() == 1 {
                return true;
            }

            let mut goal: usize = nums.len() - 1;

            for i in (0usize..=nums.len().saturating_sub(2)).rev() {
                if i.saturating_add(nums[i] as usize) >= goal {
                    goal = i;
                }
            }

            goal.eq(&0usize)
        }
    }
}
pub mod greedy {
    pub struct Solution;
    impl Solution {
        pub fn can_jump(nums: Vec<i32>) -> bool {
            if nums.len() == 1 {
                return true;
            }

            let mut goal: usize = nums.len() - 1;

            let mut current: usize = goal - 1;

            while goal > 0 {
                if current.saturating_add(nums[current] as usize) >= goal {
                    goal = current;
                    current = current.saturating_sub(1);
                    continue;
                }

                if current == 0 {
                    return false;
                }
                current = current.saturating_sub(1);
            }

            true
        }
    }
}

pub mod recursive_slow_cached {
    pub struct Solution;
    use std::collections::HashMap;
    impl Solution {
        pub fn can_jump(nums: Vec<i32>) -> bool {
            if nums.len() == 1 {
                return true;
            }
            let mut map: HashMap<usize, bool> = HashMap::with_capacity(nums.len());
            Self::backtrack(&nums, &mut map, 0)
        }

        pub fn backtrack(nums: &[i32], map: &mut HashMap<usize, bool>, i: usize) -> bool {
            if i >= nums.len() - 1 {
                return true;
            }

            let distance: usize = nums[i] as usize;

            if distance == 0 {
                map.insert(i, false);
                return false;
            }

            for jump in 1..=distance {
                if map.get(&(i + jump)).is_some_and(|x| *x == false) {
                    continue;
                }
                if Self::backtrack(nums, map, i.saturating_add(jump)) {
                    return true;
                }
            }

            map.insert(i, false);
            false
        }
    }
}
