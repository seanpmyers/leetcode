pub mod dp {
    pub struct Solution;
    impl Solution {
        pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
            let mut result: Vec<i32> = vec![0i32; temperatures.len()];

            for i in (0..temperatures.len()).rev() {
                let today = temperatures[i];
                if i + 1 >= temperatures.len() {
                    continue;
                }
                let mut j = i + 1;
                while j < temperatures.len() {
                    if temperatures[j] > today {
                        result[i] = (j - i) as i32;
                        break;
                    }
                    if result[j] == 0 {
                        break;
                    }
                    j += result[j] as usize;
                }
            }

            result
        }
    }
}
pub mod stack {
    pub struct Solution {}

    impl Solution {
        pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
            let mut answer: Vec<i32> = vec![0; temperatures.len()];
            let mut stack: Vec<(i32, usize)> = Vec::new();
            for (i, temp) in temperatures.into_iter().enumerate() {
                while !stack.is_empty() && stack[stack.len() - 1].0 < temp {
                    let top: (i32, usize) = stack.pop().unwrap();
                    answer[top.1] = (i - top.1) as i32;
                }
                stack.push((temp, i));
            }
            answer
        }
    }
}
