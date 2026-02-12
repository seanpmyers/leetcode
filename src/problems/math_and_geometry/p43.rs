pub struct Solution;
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let n1 = num1.len();
        let n2 = num2.len();
        let mut res = vec![0; n1 + n2];

        let b1 = num1.as_bytes();
        let b2 = num2.as_bytes();

        for i in (0..n1).rev() {
            for j in (0..n2).rev() {
                let idx1 = n1 - 1 - i;
                let idx2 = n2 - 1 - j;

                let digit = (b1[i] - b'0') * (b2[j] - b'0');

                res[idx1 + idx2] += digit as i32;
                res[idx1 + idx2 + 1] += res[idx1 + idx2] / 10;
                res[idx1 + idx2] %= 10;
            }
        }

        let mut result = String::new();
        let mut i = res.len() - 1;

        while i > 0 && res[i] == 0 {
            i -= 1;
        }

        while i < res.len() {
            result.push_str(&res[i].to_string());
            if i == 0 {
                break;
            }
            i -= 1;
        }

        result
    }
}
