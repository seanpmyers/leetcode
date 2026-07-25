pub mod greed {
    pub struct Solution;
    impl Solution {
        pub fn lemonade_change(bills: Vec<i32>) -> bool {
            let mut fives: u16 = 0;
            let mut tens: u16 = 0;
            for &bill in &bills {
                match bill {
                    5 => fives += 1,
                    10 => {
                        if fives == 0 {
                            return false;
                        }
                        fives -= 1;
                        tens += 1;
                    }
                    20 => {
                        if tens == 0 && fives == 0 {
                            return false;
                        }
                        if tens >= 1 && fives > 0 {
                            tens -= 1;
                            fives -= 1;
                            continue;
                        }
                        if fives >= 3 {
                            fives -= 3;
                            continue;
                        }
                        return false;
                    }
                    _ => panic!("?"),
                }
            }

            true
        }
    }
}
