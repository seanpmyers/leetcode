pub mod two_pointer {
    pub struct Solution;
    impl Solution {
        pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
            people.sort_unstable();
            let people = &people[..people.len()];
            let mut result: i32 = 0;
            let mut l: usize = 0;
            let mut r: usize = people.len() - 1;

            while l <= r {
                if l == r {
                    result += 1;
                    r = r.saturating_sub(1);
                    l += 1;
                    continue;
                }
                if people[r] + people[l] > limit {
                    result += 1;
                    r -= 1;
                    continue;
                }
                l += 1;
                r -= 1;
                result += 1;
            }

            result
        }
    }
}
