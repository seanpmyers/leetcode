pub mod linear {
    pub struct Solution;
    use std::cmp::Ordering;
    impl Solution {
        pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
            let mut result: i32 = 1;
            let mut l: usize = 0;
            let mut r: usize = 1;
            let mut previous: Ordering = Ordering::Equal;
            while r < arr.len() {
                let current: Ordering = arr[r].cmp(&arr[r - 1]);
                if !flipped(&previous, &current) {
                    result = result.max(l.abs_diff(r) as i32);
                    match current == Ordering::Equal {
                        true => l = r,
                        false => l = r - 1,
                    }
                }
                previous = current;
                r += 1;
            }
            result = result.max(l.abs_diff(r) as i32);

            result
        }
    }

    pub fn flipped(previous: &Ordering, current: &Ordering) -> bool {
        if previous == current || current == &Ordering::Equal {
            return false;
        }

        true
    }
}
pub mod quadratic {
    pub struct Solution;
    use std::cmp::Ordering;
    impl Solution {
        pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
            let mut result: i32 = 1;
            let mut l: usize = 0;
            let mut r: usize = 1;
            let mut previous: Ordering = Ordering::Equal;
            while r < arr.len() {
                let current: Ordering = arr[r].cmp(&arr[r - 1]);
                if !flipped(&previous, &current) {
                    result = result.max(l.abs_diff(r) as i32);
                    previous = Ordering::Equal;
                    l += 1;
                    r = l + 1;
                    continue;
                }
                previous = current;
                r += 1;
            }

            result = result.max(l.abs_diff(r) as i32);

            result
        }
    }

    pub fn flipped(previous: &Ordering, current: &Ordering) -> bool {
        if previous == current || current == &Ordering::Equal {
            return false;
        }

        true
    }
}
