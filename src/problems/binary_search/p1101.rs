pub mod first {
    pub struct Solution;
    impl Solution {
        pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
            let mut left: i32 = 0;
            let mut right: i32 = 0;

            weights.iter().for_each(|x| {
                left = left.max(*x);
                right += x;
            });
            let mut result: i32 = right;

            let shippable = |capacity: i32| -> bool {
                let mut ships = 1;
                let mut c = 0;
                for &weight in &weights {
                    if weight > capacity {
                        return false;
                    }
                    if c + weight > capacity {
                        c = weight;
                        ships += 1;
                        if ships > days {
                            return false;
                        }
                        continue;
                    }
                    c += weight;
                }

                ships <= days
            };

            while left <= right {
                let middle = right.midpoint(left);
                if shippable(middle) {
                    result = result.min(middle);
                    right = middle - 1;
                    continue;
                }
                left = middle + 1;
            }

            result
        }
    }
}
