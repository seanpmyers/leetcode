pub struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 1 || k == 1 {
            return nums;
        }

        let k: usize = k as usize;

        let mut result: Vec<i32> = Vec::new();

        let mut l: usize = 0;
        let mut local_max: i32 = nums[0];

        for r in 0..nums.len() {
            local_max = local_max.max(nums[r]);
            if (r - l + 1) >= k {
                result.push(local_max);
                if local_max.eq(&nums[l]) {
                    local_max = nums[r];
                    for i in l + 1..r {
                        local_max = local_max.max(nums[i]);
                    }
                }

                l = l.saturating_add(1);
            }
        }

        result
    }
    pub fn queue(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            return nums;
        }

        let k: usize = k as usize;
        let mut result: Vec<i32> = Vec::new();
        use std::collections::VecDeque;
        let mut queue: VecDeque<i32> = VecDeque::new();

        for r in 0..nums.len() {
            while let Some(back) = queue.back() {
                if *back >= nums[r] {
                    break;
                }
                queue.pop_back();
            }

            queue.push_back(nums[r]);

            if r >= k {
                if let Some(front) = queue.front() {
                    if nums[r - k] == *front {
                        queue.pop_front();
                    }
                }
            }

            if r >= k - 1 {
                if let Some(front) = queue.front() {
                    result.push(*front);
                }
            }
        }

        result
    }
}
