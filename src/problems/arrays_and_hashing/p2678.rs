pub mod first {
    pub struct Solution;
    impl Solution {
        pub fn count_seniors(details: Vec<String>) -> i32 {
            let mut result: i32 = 0;

            for detail in details.into_iter() {
                let bytes = detail.as_bytes();
                let age: i32 = ((bytes[11] - b'0') * 10) as i32 + (bytes[12] - b'0') as i32;
                if age > 60 {
                    result += 1;
                }
            }

            result
        }
    }
}
