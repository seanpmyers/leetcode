pub mod merge_sort {
    pub struct Solution;
    impl Solution {
        pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
            Self::merge_sort(&mut nums);
            nums
        }

        pub fn merge_sort(numbers: &mut [i32]) {
            if numbers.len() <= 1 {
                return;
            }
            let middle: usize = numbers.len() / 2;
            let length: usize = numbers.len();
            Self::merge_sort(&mut numbers[0..middle]);
            Self::merge_sort(&mut numbers[middle..length]);

            Self::merge(numbers);
        }

        pub fn merge(numbers: &mut [i32]) {
            if numbers.len() <= 1 {
                return;
            }

            let middle: usize = numbers.len() / 2;
            let mut result = numbers.to_vec();
            let (mut i, mut l, mut r) = (0usize, 0usize, middle);

            while l < middle && r < numbers.len() {
                if numbers[l] % 2 == 0 {
                    result[i] = numbers[l];
                    i += 1;
                    l += 1;
                    continue;
                }
                result[i] = numbers[r];
                r += 1;
                i += 1;
            }

            while l < middle {
                result[i] = numbers[l];
                i += 1;
                l += 1;
            }

            while r < numbers.len() {
                result[i] = numbers[r];
                i += 1;
                r += 1;
            }

            numbers.copy_from_slice(&result);
        }
    }
}
