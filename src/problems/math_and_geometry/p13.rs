pub mod first {
    pub struct Solution;
    impl Solution {
        pub fn roman_to_int(s: String) -> i32 {
            let s: &[u8] = s.as_bytes();
            let mut result: i32 = convert(s[0]);
            for i in 1..s.len() {
                let x: i32 = convert(s[i]);
                let y: i32 = convert(s[i - 1]);
                if y < x {
                    result -= y;
                    result += x - y;
                    continue;
                }
                result += x;
            }

            result
        }
    }

    pub fn convert(symbol: u8) -> i32 {
        match symbol {
            b'I' => 1i32,
            b'V' => 5i32,
            b'X' => 10i32,
            b'L' => 50i32,
            b'C' => 100i32,
            b'D' => 500i32,
            b'M' => 1000i32,
            _ => panic!("?"),
        }
    }
}
