pub mod merge_sort {
    pub struct Solution;
    impl Solution {
        pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            let temp: Vec<i32> = nums1[0..(m as usize)].to_vec();
            if m == 0 {
                nums1.copy_from_slice(&nums2);
                return;
            }
            let m = m as usize;
            let n = n as usize;
            let mut l: usize = 0;
            let mut r: usize = 0;
            let mut i: usize = 0;
            while l < m && r < n {
                match temp[l].cmp(&nums2[r]) {
                    std::cmp::Ordering::Greater => {
                        nums1[i] = nums2[r];
                        r += 1;
                    }
                    _ => {
                        nums1[i] = temp[l];
                        l += 1;
                    }
                }
                i += 1;
            }

            while l < m {
                nums1[i] = temp[l];
                i += 1;
                l += 1;
            }

            while r < n {
                nums1[i] = nums2[r];
                i += 1;
                r += 1;
            }
        }
    }
}
