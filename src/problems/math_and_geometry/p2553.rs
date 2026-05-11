pub mod iterative {
    pub struct Solution;

    impl Solution {
        pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
            let mut result: Vec<i32> = vec![];

            for n in nums.into_iter() {
                let mut x = n;
                let mut list: Vec<i32> = vec![];
                while x > 0 {
                    list.push(x % 10);
                    x /= 10;
                }
                list.reverse();
                result.append(&mut list);
            }

            result
        }
    }
}
