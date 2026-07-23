pub struct Solution;

use std::collections::HashMap;
#[allow(dead_code)]
struct DetectSquares {
    pub points: HashMap<usize, HashMap<usize, u32>>,
}

#[allow(dead_code)]
impl DetectSquares {
    fn new() -> Self {
        Self {
            points: HashMap::with_capacity(100),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        let x: usize = point[0] as usize;
        let y: usize = point[1] as usize;
        *self.points.entry(x).or_default().entry(y).or_insert(0u32) += 1u32;
    }

    fn count(&mut self, point: Vec<i32>) -> i32 {
        let mut result: u32 = 0;
        let x: usize = point[0] as usize;
        let y: usize = point[1] as usize;

        let Some(y_map) = self.points.get(&x) else {
            return result as i32;
        };

        for (&y2, &count) in y_map {
            let side: usize = y - y2 as usize;
            if side == 0 {
                continue;
            }
            let x3: usize = x + side;
            let x4: usize = x - side as usize;

            let get = |x: usize, y: usize| -> u32 {
                *self
                    .points
                    .get(&x)
                    .and_then(|list| list.get(&y))
                    .unwrap_or(&0)
            };

            result += count * get(x3, y) * get(x3, y2);
            result += count * get(x4, y) * get(x4, y2);
        }

        result as i32
    }
}
