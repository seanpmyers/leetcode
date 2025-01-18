use core::panic;

pub struct Solution {}
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(tokens.len());
        for symbol in tokens.into_iter() {
            match symbol.parse::<i32>() {
                Ok(number) => stack.push(number),
                Err(_) => match symbol.chars().nth(0) {
                    Some(char) => {
                        let y: i32 = stack.pop().unwrap();
                        let x: i32 = stack.pop().unwrap();
                        match char {
                            '+' => stack.push(x + y),
                            '-' => stack.push(x - y),
                            '/' => stack.push(x / y),
                            '*' => stack.push(x * y),
                            _ => panic!("?"),
                        }
                    }
                    None => panic!("?"),
                },
            }
        }
        stack.pop().unwrap()
    }
}
