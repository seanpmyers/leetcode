pub mod hashset {
    pub struct Solution;
    use std::collections::HashSet;
    impl Solution {
        pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
            let mut set: HashSet<i32> = HashSet::with_capacity(nums2.len().max(nums1.len()));
            let mut result: HashSet<i32> = HashSet::with_capacity(1000);
            for i in 0..nums1.len() {
                set.insert(nums1[i]);
            }

            for i in 0..nums2.len() {
                if set.contains(&nums2[i]) {
                    result.insert(nums2[i]);
                }
            }

            result.into_iter().collect::<Vec<i32>>()
        }
    }
}

pub mod sorting {
    pub struct Solution;
    impl Solution {
        pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
            nums1.sort();
            nums2.sort();
            let mut result: Vec<i32> = vec![];
            let (mut x, mut y): (usize, usize) = (0usize, 0usize);

            while x < nums1.len() && y < nums2.len() {
                if nums1[x] == nums2[y] {
                    let value: i32 = nums1[x];
                    result.push(value);
                    while x < nums1.len() && nums1[x] == value {
                        x += 1;
                    }
                    while y < nums2.len() && nums2[y] == value {
                        y += 1;
                    }
                    continue;
                }

                if nums1[x] < nums2[y] {
                    x += 1;
                    continue;
                }

                if nums2[y] < nums1[x] {
                    y += 1;
                    continue;
                }
            }

            result
        }
    }
}
