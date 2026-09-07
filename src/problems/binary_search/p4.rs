pub mod second {
    pub struct Solution;
    impl Solution {
        pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
            if nums1.len() > nums2.len() {
                std::mem::swap(&mut nums1, &mut nums2);
            }
            let m: usize = nums1.len();
            let n: usize = nums2.len();
            let even: bool = (m + n) % 2 == 0;
            let half: usize = (m + n + 1) / 2;

            let mut l: usize = 0;
            let mut r: usize = m;

            loop {
                let x_cut: usize = r.midpoint(l);
                let y_cut: usize = half - x_cut;

                let x_left: i32 = if x_cut > 0 {
                    nums1[x_cut - 1]
                } else {
                    i32::MIN
                };
                let y_left: i32 = if y_cut > 0 {
                    nums2[y_cut - 1]
                } else {
                    i32::MIN
                };

                let x_right: i32 = if x_cut < m { nums1[x_cut] } else { i32::MAX };
                let y_right: i32 = if y_cut < n { nums2[y_cut] } else { i32::MAX };

                if x_left <= y_right && y_left <= x_right {
                    if even {
                        return (x_left.max(y_left) + y_right.min(x_right)) as f64 / 2f64;
                    }
                    return x_left.max(y_left) as f64;
                }
                if x_left > y_right {
                    r = x_cut - 1;
                    continue;
                }
                l = x_cut + 1;
            }
        }
    }
}
pub mod first {
    pub struct Solution;

    impl Solution {
        pub fn middle(l: usize, r: usize) -> usize {
            (l + r) / 2
        }

        pub fn middle_value(l: f64, r: f64) -> f64 {
            (l + r) / 2f64
        }

        pub fn middle_of_array(list: &Vec<i32>) -> f64 {
            match list.len() % 2 == 0 {
                true => {
                    let middle_index = Self::middle(0, list.len());
                    Self::middle_value(middle_index as f64, middle_index.saturating_add(1) as f64)
                }
                false => list[Self::middle(0, list.len())] as f64,
            }
        }
        pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
            if nums1.is_empty() {
                return Self::middle_of_array(&nums2);
            }

            if nums2.is_empty() {
                return Self::middle_of_array(&nums1);
            }
            let mut a: &Vec<i32> = &nums1;
            let mut b: &Vec<i32> = &nums2;

            if b.len() < a.len() {
                std::mem::swap(&mut a, &mut b);
            }

            let mut l: usize = 0;
            let mut r: usize = a.len();

            let total_len: usize = nums1.len() + nums2.len();
            let half: usize = Self::middle(1, total_len);

            let result: f64 = 0.0f64;

            while l <= r {
                let i: usize = Self::middle(l, r);
                let j: usize = half - i;

                let a_left: i32 = match i > 0 {
                    true => a[i - 1],
                    false => i32::MIN,
                };
                let a_right: i32 = match i < a.len() {
                    true => a[i],
                    false => i32::MAX,
                };

                let b_left: i32 = match j > 0 {
                    true => b[j - 1],
                    false => i32::MIN,
                };
                let b_right: i32 = match j < b.len() {
                    true => b[j],
                    false => i32::MAX,
                };

                if a_left <= b_right && b_left <= a_right {
                    if total_len % 2 != 0 {
                        return a_left.max(b_left) as f64;
                    }
                    return (a_left.max(b_left) + a_right.min(b_right)) as f64 / 2.0f64;
                } else if a_left > b_right {
                    r = i - 1;
                } else {
                    l = i + 1;
                }
            }

            result
        }
    }
}
