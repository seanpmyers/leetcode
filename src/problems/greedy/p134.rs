pub struct Solution;
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if cost.iter().sum::<i32>() > gas.iter().sum::<i32>() {
            return -1i32;
        }
        let mut result: usize = 0;
        let mut total: i32 = 0;
        for i in 0..gas.len() {
            let net: i32 = gas[i] - cost[i];
            total += net;
            if total < 0 {
                result = i + 1;
                total = 0;
            }
        }

        result as i32
    }
}
