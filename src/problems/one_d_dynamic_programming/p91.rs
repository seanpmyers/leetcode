pub mod fibonacci {
    pub struct Solution;
    impl Solution {
        pub fn num_decodings(s: String) -> i32 {
            let bytes = s.as_bytes();

            if bytes[0] == b'0' {
                return 0;
            }

            let mut first = 0;
            let mut second = 1;

            for i in 0..bytes.len() {
                let mut current = 0;

                if bytes[i] != b'0' {
                    current += second;
                }

                if i.checked_sub(1).is_some_and(|idx| idx < bytes.len()) {
                    let prev_digit = bytes[i.saturating_sub(1)];
                    let curr_digit = bytes[i];

                    let is_valid_pair = match prev_digit {
                        b'1' => true,
                        b'2' => curr_digit <= b'6',
                        _ => false,
                    };

                    if is_valid_pair {
                        current += first;
                    }
                }

                first = second;
                second = current;

                if second == 0 {
                    return 0;
                }
            }

            second
        }
    }
}
