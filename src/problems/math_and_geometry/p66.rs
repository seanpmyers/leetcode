pub mod option_1 {
    pub struct Solution {}
    impl Solution {
        pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
            for i in (0..digits.len()).rev() {
                if digits[i] == 9 {
                    digits[i] = 0;
                    if i == 0 {
                        digits.insert(0, 1);
                        return digits;
                    }
                    continue;
                }
                digits[i] += 1;
                return digits;
            }
            digits
        }
    }
}

pub mod option_2 {
    pub struct Solution {}
    impl Solution {
        pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
            for i in (0..digits.len()).rev() {
                if digits[i] != 9 {
                    digits[i] += 1;
                    return digits;
                }

                digits[i] = 0;
                if i != 0 {
                    continue;
                }

                digits[i] += 1;
                return digits;
            }

            digits
        }
    }
}

pub mod option_3 {
    pub struct Solution {}
    impl Solution {
        pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
            for i in (0..digits.len()).rev() {
                let Some(x) = digits.get_mut(i) else {
                    break;
                };

                if x.saturating_add(1) <= 9 {
                    *x = x.saturating_add(1);
                    return digits;
                }

                if i == 0 {
                    *x = 0;
                    digits.insert(0, 1);
                    return digits;
                }

                if x.saturating_add(1) <= 9 {
                    *x = x.saturating_add(1);
                    return digits;
                }

                *x = 0;
            }

            digits
        }
    }
}
