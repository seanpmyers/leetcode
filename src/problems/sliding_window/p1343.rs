pub mod linear {
    pub struct Solution;
    impl Solution {
        pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
            let mut result: i32 = 0;
            let ksize: usize = k as usize;
            let mut l: usize = 0;
            let mut r: usize = 0;
            let mut current: i32 = 0;

            while r < arr.len() {
                while l.abs_diff(r) < ksize {
                    current += arr[r];
                    r += 1;
                }

                if current >= threshold * k {
                    result += 1;
                }

                current -= arr[l];
                l += 1;
            }

            result
        }
    }
}
