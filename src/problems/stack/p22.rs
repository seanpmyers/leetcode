pub struct Solution {}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut stack: Vec<char> = Vec::with_capacity(n as usize);
        backtrack(n, 0, 0, &mut result, &mut stack);
        result
    }
}

pub fn backtrack(n: i32, open: i32, closed: i32, result: &mut Vec<String>, stack: &mut Vec<char>) {
    if open == closed && open == n {
        result.push(stack.iter().collect::<String>());
        return;
    }

    if open < n {
        stack.push('(');
        backtrack(n, open + 1, closed, result, stack);
        stack.pop();
    }

    if closed < open {
        stack.push(')');
        backtrack(n, open, closed + 1, result, stack);
        stack.pop();
    }
}
