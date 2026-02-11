pub struct Solution;
impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        if x == 0f64 {
            return x;
        }
        if n == 0 {
            return 1f64;
        }

        let mut result: f64 = 1f64;
        let mut power: i64 = (n as i64).abs();

        while power > 0 {
            // power & 1 == 1
            if power % 2 == 1 {
                result *= x;
            }

            x *= x;
            power >>= 1;
            // power /= 2;
        }

        if n >= 0 { result } else { 1.0f64 / result }
    }
}
