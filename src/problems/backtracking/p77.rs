pub struct Solution;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let k: usize = k as usize;
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(100usize);
        Self::backtrack(&mut result, &mut vec![], n, 1i32, k);
        result
    }

    pub fn backtrack(result: &mut Vec<Vec<i32>>, list: &mut Vec<i32>, n: i32, i: i32, k: usize) {
        if list.len() == k {
            result.push(list.to_vec());
            return;
        }

        if i > n {
            return;
        }

        for x in i..=n {
            list.push(x);
            Self::backtrack(result, list, n, x + 1, k);
            list.pop();
        }
    }
}
