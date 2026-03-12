pub mod first_attempt {
    pub struct Solution;
    impl Solution {
        pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
            if nums.is_empty() {
                return 0i32;
            }
            let mut swap: usize = nums.len() - 1;
            while swap > 0 && nums[swap] == val {
                swap -= 1;
            }
            if swap == 0 && nums[0] == val {
                return 0;
            }
            let mut k: i32 = 0;
            for i in 0..nums.len() {
                if nums[i] != val {
                    k += 1;
                    continue;
                }
                if swap < i {
                    break;
                }
                nums.swap(i, swap);
                k += 1;
                swap -= 1;
                while swap > 0 && nums[swap] == val {
                    swap -= 1;
                }
                if swap == 0 {
                    return k;
                }
            }

            k
        }
    }
}

pub mod clean {
    pub struct Solution;
    impl Solution {
        pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
            let mut k: usize = 0;

            for i in 0..nums.len() {
                if nums[i] != val {
                    nums.swap(k, i);
                    k += 1;
                }
            }

            k as i32
        }
    }
}
