pub struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s: Vec<char> = s.chars().collect::<Vec<char>>();
        let mut stack: Vec<char> = Vec::new();
        for i in 0..s.len() {
            let x: char = s[i];
            if !handle_char(&mut stack, x) {
                return false;
            }
        }
        if !stack.is_empty() {
            return false;
        }
        true
    }
}

pub fn handle_char(stack: &mut Vec<char>, x: char) -> bool {
    match x {
        ']' => {
            if !is_match(x, stack.pop()) {
                return false;
            }
        }
        '}' => {
            if !is_match(x, stack.pop()) {
                return false;
            }
        }
        ')' => {
            if !is_match(x, stack.pop()) {
                return false;
            }
        }
        '(' => stack.push(x),
        '{' => stack.push(x),
        '[' => stack.push(x),
        _ => return false,
    }
    true
}

pub fn is_match(x: char, y: Option<char>) -> bool {
    let Some(y) = y else {
        return false;
    };
    match x {
        ']' => return y == '[',
        ')' => return y == '(',
        '}' => return y == '{',
        _ => return false,
    }
}
