pub mod problems;

fn main() {
    println!("Leetcode");
}
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max: i32 = 0;
        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;

        while l < r {
            let min = height[l].min(height[r]);
            max = max.max(min * (r - l) as i32);
            if height[l] < height[r] {
                l = l.saturating_add(1);
                continue;
            }
            r = r.saturating_sub(1);
        }

        max
    }
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l: usize = 0;
        let mut r: usize = numbers.len() - 1;
        while l < r {
            match target.cmp(&(numbers[l] + numbers[r])) {
                std::cmp::Ordering::Greater => l = l.saturating_add(1),
                std::cmp::Ordering::Equal => return vec![l as i32 + 1, r as i32 + 1],
                std::cmp::Ordering::Less => r = r.saturating_sub(1),
            }
        }
        vec![]
    }
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        const LOWER_A: u8 = b'a';
        const LOWER_Z: u8 = b'z';
        const UPPER_A: u8 = b'A';
        const UPPER_Z: u8 = b'Z';
        const ZERO: u8 = b'0';
        const NINE: u8 = b'9';
        pub fn to_lower(c: u8) -> u8 {
            if c >= LOWER_A {
                return c;
            }

            c + (LOWER_A - UPPER_A)
        }
        pub fn relevant(c: u8) -> bool {
            let upper_alpha: bool = c >= UPPER_A && c <= UPPER_Z;
            let lower_alpha: bool = c >= LOWER_A && c <= LOWER_Z;
            let number: bool = c >= ZERO && c <= NINE;
            upper_alpha || lower_alpha || number
        }

        let mut l: usize = 0;
        let mut r: usize = s.len() - 1;

        let s = s.as_bytes();

        while l < r {
            while l < r && !relevant(s[l] as u8) {
                l = l.saturating_add(1);
            }
            while l < r && !relevant(s[r] as u8) {
                r = r.saturating_sub(1);
            }
            if to_lower(s[l]) != to_lower(s[r]) {
                return false;
            }
            l = l.saturating_add(1);
            r = r.saturating_sub(1);
        }

        true
    }
}
