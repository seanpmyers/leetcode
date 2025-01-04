pub mod problems;

fn main() {
    println!("Leetcode");
}

pub struct Solution;
impl Solution {
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
