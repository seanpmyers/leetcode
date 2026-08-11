pub mod binary_search {
    pub struct Solution;
    impl Solution {
        pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
            let k: usize = k as usize;
            let mut l: usize = 0;
            let mut r: usize = arr.len() - k;

            while l < r {
                let middle: usize = r.midpoint(l);
                let a: i32 = x - arr[middle];
                let b: i32 = arr[middle + k] - x;

                if a > b {
                    l = middle + 1;
                    continue;
                }
                r = middle;
            }

            arr[l..(l + k)].to_vec()
        }
    }
}
pub mod first {
    pub struct Solution;
    impl Solution {
        pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
            let mut l: usize = 0;
            let mut r: usize = arr.len() - 1;
            let k: usize = k as usize;
            while l < r && r.abs_diff(l) > k - 1 {
                let a: u32 = arr[l].abs_diff(x);
                let b: u32 = arr[r].abs_diff(x);
                if a <= b {
                    r -= 1;
                    continue;
                }
                l += 1;
            }

            arr[l..=r].to_vec()
        }
    }
}
