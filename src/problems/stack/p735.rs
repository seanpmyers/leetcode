pub mod first_attempt {
    pub struct Solution;
    impl Solution {
        pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
            let mut stack: Vec<i32> = Vec::new();

            for a in asteroids.into_iter() {
                if stack.is_empty() || a > 0 {
                    stack.push(a);
                    continue;
                }
                if stack.last().is_some_and(|x| x * a > 0 || *x < 0 && a > 0) {
                    stack.push(a);
                    continue;
                }
                while stack.last().is_some_and(|x| x * a < 0) {
                    let top = stack.pop().unwrap();
                    if a.abs() == top.abs() {
                        break;
                    }
                    if top.abs() > a.abs() {
                        stack.push(top);
                        break;
                    }
                    if (stack.is_empty() || stack.last().is_some_and(|x| x * a > 0))
                        && a.abs() > top.abs()
                    {
                        stack.push(a);
                        break;
                    }
                }
            }

            stack
        }
    }
}
