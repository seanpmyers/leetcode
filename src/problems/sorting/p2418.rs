pub mod merge_sort {
    pub struct Solution;
    impl Solution {
        pub fn sort_people(mut names: Vec<String>, mut heights: Vec<i32>) -> Vec<String> {
            Self::merge_sort(&mut names, &mut heights);
            names
        }

        pub fn merge_sort(names: &mut [String], heights: &mut [i32]) {
            if names.len() <= 1 {
                return;
            }

            let length = names.len();
            let middle = length / 2;

            Self::merge_sort(&mut names[0..middle], &mut heights[0..middle]);
            Self::merge_sort(&mut names[middle..length], &mut heights[middle..length]);

            Self::merge(names, heights);
        }

        pub fn merge(names: &mut [String], heights: &mut [i32]) {
            if names.len() <= 1 {
                return;
            }
            let length: usize = names.len();
            let middle: usize = length / 2;
            let mut result: Vec<(String, i32)> = Vec::with_capacity(length);
            let mut l: usize = 0;
            let mut r: usize = middle;

            while l < middle && r < length {
                match heights[l] > heights[r] {
                    true => {
                        result.push((names[l].clone(), heights[l]));
                        l += 1;
                    }
                    false => {
                        result.push((names[r].clone(), heights[r]));
                        r += 1;
                    }
                }
            }

            while r < length {
                result.push((names[r].clone(), heights[r]));
                r += 1;
            }

            while l < middle {
                result.push((names[l].clone(), heights[l]));
                l += 1;
            }

            for i in 0..result.len() {
                names[i] = result[i].0.clone();
                heights[i] = result[i].1;
            }
        }
    }
}
