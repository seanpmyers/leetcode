pub trait Solution {
    fn new() -> MinStack;
    fn push(&mut self, val: i32);
    fn pop(&mut self);
    fn top(&self) -> i32;
    fn get_min(&self) -> i32;
}
pub struct MinStack {
    pub min: Vec<i32>,
    pub stack: Vec<i32>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        MinStack {
            min: vec![],
            stack: vec![],
        }
    }
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        match self.min.is_empty() {
            true => self.min.push(val),
            false => {
                if self.min[self.min.len() - 1] >= val {
                    self.min.push(val);
                }
            }
        }
    }
    fn pop(&mut self) {
        if let Some(top) = self.stack.pop() {
            if self.min[self.min.len() - 1] == top {
                self.min.pop();
            }
        }
    }
    fn top(&self) -> i32 {
        if self.stack.is_empty() {
            return -1i32;
        }
        self.stack[self.stack.len() - 1]
    }
    fn get_min(&self) -> i32 {
        if self.min.is_empty() {
            return -1i32;
        }
        self.min[self.min.len() - 1]
    }
}
