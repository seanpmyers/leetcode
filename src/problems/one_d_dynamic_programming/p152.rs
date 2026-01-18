pub mod kadane {
    pub struct Solution;
    impl Solution {
        pub fn max_product(nums: Vec<i32>) -> i32 {
            if nums.len() == 1 {
                return nums[0];
            }

            let mut result: i32 = nums[0];

            let mut max: i32 = 1i32;
            let mut min: i32 = 1i32;

            for number in nums.iter() {
                let product: i32 = number * max;

                max = product.max(*number).max(number * min);
                min = product.min(*number).min(number * min);

                result = result.max(max);
            }

            result
        }
    }
}

pub mod first_attempt {
    pub struct Solution;
    impl Solution {
        pub fn max_product(nums: Vec<i32>) -> i32 {
            if nums.len() == 1 {
                return nums[0];
            }

            let mut result: i32 = nums.iter().product::<i32>();
            let len = nums.len();

            for i in 0..len {
                let mut product: i32 = nums[i];
                let mut reverse: i32 = nums[len.saturating_sub(1 + i)];
                result = result.max(product).max(reverse);
                for j in i.saturating_add(1)..len {
                    product *= nums[j];
                    reverse *= nums[len.saturating_sub(1 + j)];
                    result = result.max(product).max(reverse);
                }
            }

            result
        }
    }
}
