pub struct Solution;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max_area: i32 = 0;
        let mut stack: Vec<(i32, usize)> = vec![];
        let len: usize = heights.len();
        for (i, h) in heights.into_iter().enumerate() {
            let mut start: usize = i;
            while !stack.is_empty() && stack[stack.len() - 1].0 > h {
                let top = stack.pop().unwrap();
                max_area = max_area.max(top.0 * (i - top.1) as i32);
                start = top.1;
            }
            stack.push((h, start));
        }
        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            max_area = max_area.max(top.0 * (len - top.1) as i32);
        }
        max_area
    }
}
