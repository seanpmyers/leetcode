pub struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut result: i32 = 0_i32;
        let mut stack: Vec<usize> = vec![];
        for i in 0..height.len() {
            while !stack.is_empty() && height[i] >= height[stack[stack.len() - 1]] {
                let middle: i32 = height[stack.pop().unwrap()];
                if !stack.is_empty() {
                    let right: i32 = height[i];
                    let left: i32 = height[stack[stack.len() - 1]];
                    let h: i32 = right.min(left) - middle;
                    let w: usize = i - stack[stack.len() - 1] - 1usize;
                    result += h * w as i32;
                }
            }
            stack.push(i);
        }
        result
    }
}
