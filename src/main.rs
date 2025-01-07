pub mod problems;

fn main() {
    println!("Leetcode");
}

pub struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut result: i32 = 0i32;
        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;
        let mut l_max: i32 = height[l];
        let mut r_max: i32 = height[r];

        while l < r {
            if l_max < r_max {
                l += 1;
                let left = height[l];
                l_max = l_max.max(left);
                result += l_max - left;
                continue;
            }
            r -= 1;
            let right = height[r];
            r_max = r_max.max(right);
            result += r_max - right;
        }
        result
    }
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;
        while l < r {
            let left: i32 = height[l];
            let right: i32 = height[r];
            let h = left.min(right);
            let w = r - l;
            result = result.max(h * w as i32);
            if left < right {
                l += 1;
                continue;
            }
            r -= 1;
        }
        result
    }
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                '}' => match stack.pop() {
                    Some('{') => {}
                    _ => return false,
                },
                ')' => match stack.pop() {
                    Some('(') => {}
                    _ => return false,
                },
                ']' => match stack.pop() {
                    Some('[') => {}
                    _ => return false,
                },
                _ => panic!("Invalid input"),
            }
        }
        stack.is_empty()
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut result: HashSet<Vec<i32>> = HashSet::new();

        for i in 0..nums.len() {
            let mut l: usize = i + 1;
            let mut r: usize = nums.len() - 1;
            while l < r {
                match (nums[i] + nums[l] + nums[r]).cmp(&0) {
                    std::cmp::Ordering::Less => l += 1,
                    std::cmp::Ordering::Equal => {
                        result.insert(vec![nums[i], nums[l], nums[r]]);
                        l += 1;
                        r -= 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                    }
                    std::cmp::Ordering::Greater => r -= 1,
                }
            }
        }

        result.into_iter().collect()
    }

    pub fn two_sum_2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        //array is sorted
        pub fn middle(s: usize, e: usize) -> usize {
            s + (e - s) / 2
        }
        let (mut l, mut r): (usize, usize) = (0, numbers.len() - 1);
        while l < r {
            let middle: usize = middle(l, r);
            match target.cmp(&(numbers[l] + numbers[r])) {
                std::cmp::Ordering::Equal => return vec![l as i32 + 1, r as i32 + 1],
                std::cmp::Ordering::Greater => {
                    l = match numbers[middle] + numbers[r] < target {
                        true => middle + 1,
                        false => l + 1,
                    }
                }
                std::cmp::Ordering::Less => {
                    r = match numbers[middle] + numbers[l] > target {
                        true => middle - 1,
                        false => r - 1,
                    }
                }
            }
        }
        vec![0, 0]
    }
    pub fn is_palindrome(s: String) -> bool {
        const LOWER_A: u8 = b'a' as u8;
        const LOWER_Z: u8 = b'z' as u8;
        const UPPER_A: u8 = b'A' as u8;
        const UPPER_Z: u8 = b'Z' as u8;
        const ZERO: u8 = b'0' as u8;
        const NINE: u8 = b'9' as u8;
        pub fn is_valid_char(c: u8) -> bool {
            c >= LOWER_A && c <= LOWER_Z || c >= UPPER_A && c <= UPPER_Z || c >= ZERO && c <= NINE
        }
        pub fn to_lowercase(mut c: u8) -> u8 {
            if c >= UPPER_A && c <= UPPER_Z {
                c = c + (LOWER_A - UPPER_A);
            }
            c
        }

        let mut l: usize = 0;
        let mut r: usize = s.len() - 1;
        let chars = s.as_bytes();
        while l < r {
            while l < r && !is_valid_char(chars[l]) {
                l = l.saturating_add(1);
            }
            while r > l && !is_valid_char(chars[r]) {
                r = r.saturating_sub(1);
            }

            if to_lowercase(chars[l]) != to_lowercase(chars[r]) {
                return false;
            }
            l = l.saturating_add(1);
            r = r.saturating_sub(1);
        }

        true
    }

    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut longest: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for n in nums.into_iter() {
            if !map.contains_key(&n) {
                let previous = map.get(&(n - 1)).unwrap_or(&0).clone();
                let next = map.get(&(n + 1)).unwrap_or(&0).clone();
                let count = 1 + previous + next;
                map.insert(n, count);
                map.entry(n - previous)
                    .and_modify(|v| *v = count)
                    .or_insert(count);
                map.entry(n + next)
                    .and_modify(|v| *v = count)
                    .or_insert(count);
                longest = longest.max(count);
            }
        }

        longest
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        let mut row: Vec<HashSet<u32>> = vec![HashSet::with_capacity(9); 9];
        let mut column: Vec<HashSet<u32>> = vec![HashSet::with_capacity(9); 9];
        let mut block: Vec<HashSet<u32>> = vec![HashSet::with_capacity(9); 9];
        for r in 0..board.len() {
            for c in 0..board[r].len() {
                match &board[r][c].to_digit(10) {
                    Some(x) => {
                        if !row[r].insert(*x) {
                            return false;
                        }
                        if !column[c].insert(*x) {
                            return false;
                        }
                        if !block[3 * (r / 3) + (c / 3)].insert(*x) {
                            return false;
                        }
                    }
                    None => {}
                }
            }
        }
        true
    }

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let length: usize = nums.len() - 1;
        let mut result: Vec<i32> = vec![1i32; length + 1];
        let (mut l, mut r): (i32, i32) = (1i32, 1i32);
        for i in 0..length {
            result[i] *= l;
            result[length - i] *= r;
            l *= nums[i];
            r *= nums[length - i];
        }
        result[0] *= r;
        result[length] *= l;
        result
    }

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::{BinaryHeap, HashMap};
        let mut result: Vec<i32> = vec![];
        let mut map: HashMap<i32, u16> = HashMap::with_capacity(nums.len());
        let mut heap: BinaryHeap<(u16, i32)> = BinaryHeap::with_capacity(nums.len());
        nums.into_iter().for_each(|n| {
            map.entry(n).and_modify(|x| *x += 1).or_insert(1);
        });
        for (k, v) in map.into_iter() {
            heap.push((v, k));
        }
        for _ in 0..k {
            result.push(heap.pop().unwrap().1);
        }
        result
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut map: HashMap<[u16; 26], Vec<String>> = HashMap::new();
        const A_OFFSET: usize = b'a' as usize;
        for s in strs.into_iter() {
            let mut count = [0u16; 26];
            s.chars().for_each(|x| {
                count[x as usize - A_OFFSET] += 1;
            });
            map.entry(count)
                .and_modify(|v| v.push(s.clone()))
                .or_insert(vec![s.clone()]);
        }
        map.into_values().collect()
    }
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (i, n) in nums.into_iter().enumerate() {
            if let Some(x) = map.get(&n) {
                return vec![i as i32, *x];
            }
            map.insert(target - n, i as i32);
        }
        vec![0i32, 1i32]
    }
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let (mut x, mut y): ([u16; 26], [u16; 26]) = ([0u16; 26usize], [0u16; 26usize]);
        let start = 'a' as usize;
        for (a, b) in s.chars().zip(t.chars()) {
            x[a as usize - start] += 1;
            y[b as usize - start] += 1;
        }
        x == y
    }
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());
        nums.into_iter().any(|x| !set.insert(x))
    }
}
