pub struct Solution;
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n: usize = position.len();
        if n <= 1 {
            return n as i32;
        }
        let mut result: i32 = n as i32;
        let mut stack: Vec<(i32, i32)> = Vec::with_capacity(n);
        for i in 0..n {
            let mut fleet: i32 = 0;
            let (mut p, mut s): (i32, i32) = (position[i], speed[i]);
            while !stack.is_empty() {
                if let Some(top) = stack.pop() {
                    let t_update = top.0 + top.1;
                    if t_update == target {
                        fleet += 1;
                        if p + s == target {
                            fleet += 1;
                            continue;
                        } else {
                            p += s;
                        }
                    }
                }
            }
            stack.push((p, s));
        }
        result
    }
}
