pub struct Solution;
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n: usize = position.len();
        if n <= 1 {
            return n as i32;
        }
        let mut values: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
        values.sort_by(|x, y| y.0.cmp(&x.0));
        let mut stack: Vec<f32> = vec![];
        for (p, s) in values.into_iter() {
            stack.push((target - p) as f32 / s as f32);
            if stack.len() >= 2 && stack[stack.len() - 1] <= stack[stack.len() - 2] {
                stack.pop();
            }
        }
        stack.len() as i32
    }
}
