pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::with_capacity(s.len() * 2);

        Self::backtrack(0, 0, s.as_bytes(), &mut result, vec![]);

        result
    }

    pub fn backtrack(
        start: usize,
        end: usize,
        input: &[u8],
        result: &mut Vec<Vec<String>>,
        mut current: Vec<String>,
    ) {
        if end >= input.len() {
            if end == start {
                result.push(current.clone());
            }
            return;
        }

        if Self::is_palindrome(input, start, end) {
            let sub_string: String = input[start..=end]
                .iter()
                .map(|x| *x as char)
                .collect::<String>();
            current.push(sub_string);
            Self::backtrack(
                end.saturating_add(1),
                end.saturating_add(1),
                input,
                result,
                current.clone(),
            );
            current.pop();
        }

        Self::backtrack(start, end.saturating_add(1), input, result, current);
    }

    pub fn is_palindrome(input: &[u8], mut start: usize, mut end: usize) -> bool {
        while start < end {
            if input[start] != input[end] {
                return false;
            }

            start = start.saturating_add(1);
            end = end.saturating_sub(1);
        }

        true
    }
}
