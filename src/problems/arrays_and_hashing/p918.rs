pub mod kadane {
    pub struct Solution;
    impl Solution {
        pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
            let mut total: i32 = 0;
            let mut gmax: i32 = nums[0];
            let mut gmin: i32 = nums[0];
            let mut cmax: i32 = 0;
            let mut cmin: i32 = 0;

            for &n in &nums {
                cmax = n.max(cmax + n);
                cmin = n.min(cmin + n);

                gmin = gmin.min(cmin);
                gmax = gmax.max(cmax);

                total += n;
            }

            if gmax > 0 {
                gmax.max(total - gmin)
            } else {
                gmax
            }
        }
    }
}
