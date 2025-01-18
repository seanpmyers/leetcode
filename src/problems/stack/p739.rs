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
