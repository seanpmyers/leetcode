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
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        if nums.len() == 2 {
            return nums[0];
        }
        let mut slow: usize = 0;
        let mut fast: usize = 0;
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }
        let mut slow2: usize = 0;
        loop {
            slow = nums[slow] as usize;
            slow2 = nums[slow2] as usize;
            if slow == slow2 {
                return slow as i32;
            }
        }
    }

    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result: ListNode = ListNode::new(0i32);
        let mut current = &mut result;
        let mut carry: i32 = 0;
        pub fn digit_calc(digit: &mut i32, carry: &mut i32) {
            *carry = *digit / 10;
            *digit = *digit % 10;
        }
        while l1.is_some() || l2.is_some() {
            match (l1.clone(), l2.clone()) {
                (None, None) => {}
                (None, Some(l2_n)) => {
                    let next = l2_n.next.clone();
                    let mut digit = carry + l2_n.val;
                    digit_calc(&mut digit, &mut carry);
                    let new_node = Box::new(ListNode::new(digit));
                    current.next = Some(new_node);
                    current = current.next.as_mut().unwrap();
                    l2 = next;
                }
                (Some(l1_n), None) => {
                    let next = l1_n.next.clone();
                    let mut digit = carry + l1_n.val;
                    digit_calc(&mut digit, &mut carry);
                    let new_node = Box::new(ListNode::new(digit));
                    current.next = Some(new_node);
                    current = current.next.as_mut().unwrap();
                    l1 = next;
                }
                (Some(l1_n), Some(l2_n)) => {
                    let next_1 = l1_n.next.clone();
                    let next_2 = l2_n.next.clone();
                    let mut digit = carry + l1_n.val + l2_n.val;
                    digit_calc(&mut digit, &mut carry);
                    let new_node = Box::new(ListNode::new(digit));
                    current.next = Some(new_node);
                    current = current.next.as_mut().unwrap();
                    l1 = next_1;
                    l2 = next_2;
                }
            }
        }
        if carry != 0 {
            let mut digit = 0 + carry;
            digit_calc(&mut digit, &mut carry);
            let new_node = Box::new(ListNode::new(digit));
            current.next = Some(new_node);
            _ = current.next.as_mut().unwrap();
        }

        result.next
    }
    pub fn is_happy(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        pub fn process_digits(mut n: i32) -> i32 {
            let mut result: i32 = 0;
            while n != 0 {
                result += (n % 10).pow(2);
                n /= 10;
            }
            result
        }
        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::new();
        let mut happy: i32 = n;
        while happy != 1 {
            let current: i32 = process_digits(happy);
            if !set.insert(current) {
                return false;
            }
            happy = current;
        }
        true
    }
    pub fn character_replacement(s: String, k: i32) -> i32 {
        if s.is_empty() {
            return 0;
        }
        use std::collections::HashMap;
        let mut map: HashMap<u8, i32> = HashMap::new();
        let s: Vec<u8> = s.into_bytes();
        let mut result: i32 = 0;
        let mut max: i32 = 0;
        let length: usize = s.len();
        let mut l: usize = 0;
        for i in 0..s.len() {
            let count = map.entry(s[i]).and_modify(|x| *x += 1i32).or_insert(1i32);
            max = max.max(*count);
            while (i - l + 1) as i32 - max > k {
                map.entry(s[l]).and_modify(|x| *x -= 1);
                l += 1;
            }
            result = result.max((i - l + 1) as i32);
        }
        result.min(length as i32)
    }
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut profit: i32 = 0;
        let mut min: i32 = prices[0];
        for p in prices.into_iter().skip(1) {
            profit = profit.max(p - min);
            min = min.min(p);
        }
        profit
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(r) => {
                pub fn dive(n: Rc<RefCell<TreeNode>>, max: Option<i32>, min: Option<i32>) -> bool {
                    let n_val = n.as_ref().borrow().val;
                    if let Some(l) = min {
                        if n_val <= l {
                            return false;
                        }
                    }
                    if let Some(r) = max {
                        if n_val >= r {
                            return false;
                        }
                    }

                    if let Some(l) = n.as_ref().borrow().left.clone() {
                        if !dive(l, Some(n_val), min) {
                            return false;
                        }
                    }
                    if let Some(r) = n.as_ref().borrow().right.clone() {
                        if !dive(r, max, Some(n_val)) {
                            return false;
                        }
                    }
                    true
                }
                dive(r, None, None)
            }
            None => true,
        }
    }
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(r) => {
                let mut stack: Vec<(Rc<RefCell<TreeNode>>, usize)> = vec![(r, 1)];
                let mut result: Vec<i32> = vec![];
                while let Some((top, depth)) = stack.pop() {
                    if result.len() < depth {
                        result.push(top.as_ref().borrow().val);
                    }
                    if top.as_ref().borrow().left.is_some() {
                        stack.push((top.as_ref().borrow_mut().left.take().unwrap(), depth + 1));
                    }
                    if top.as_ref().borrow().right.is_some() {
                        stack.push((top.as_ref().borrow_mut().right.take().unwrap(), depth + 1));
                    }
                }
                result
            }
            None => vec![],
        }
    }
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count: i32 = 0;
        match root {
            Some(r) => {
                pub fn recurse(count: &mut i32, node: Rc<RefCell<TreeNode>>, max: i32) {
                    if node.borrow().val >= max {
                        *count += 1;
                    }
                    let max = max.max(node.borrow().val);
                    match node.borrow_mut().left.take() {
                        Some(l) => recurse(count, l, max),
                        None => {}
                    }
                    match node.borrow_mut().right.take() {
                        Some(r) => recurse(count, r, max),
                        None => {}
                    }
                }
                let max: i32 = r.borrow().val;
                recurse(&mut count, r, max);
            }
            None => {}
        }
        count
    }
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max: i32 = 0;
        match root {
            Some(r) => {
                let mut stack: Vec<(Rc<RefCell<TreeNode>>, i32)> = vec![(r, 1)];
                while let Some((n, depth)) = stack.pop() {
                    max = max.max(depth);
                    match n.as_ref().borrow_mut().left.take() {
                        Some(l) => stack.push((l, depth + 1)),
                        None => {}
                    }
                    match n.as_ref().borrow_mut().right.take() {
                        Some(r) => stack.push((r, depth + 1)),
                        None => {}
                    }
                }
            }
            None => return max,
        }
        max
    }
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut stack: Vec<(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>)> =
            vec![(p, q)];

        while let Some((l, r)) = stack.pop() {
            match (l, r) {
                (None, None) => continue,
                (None, Some(_)) => return false,
                (Some(_), None) => return false,
                (Some(x), Some(y)) => {
                    if x.as_ref().borrow().val != y.as_ref().borrow().val {
                        return false;
                    }
                    stack.push((
                        x.as_ref().borrow_mut().left.take(),
                        y.as_ref().borrow_mut().left.take(),
                    ));
                    stack.push((
                        x.as_ref().borrow_mut().right.take(),
                        y.as_ref().borrow_mut().right.take(),
                    ));
                }
            }
        }
        true
    }
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = Vec::with_capacity(temperatures.len());
        let mut result: Vec<i32> = vec![0; temperatures.len()];
        for (i, t) in temperatures.into_iter().enumerate() {
            while !stack.is_empty() && stack[stack.len() - 1].0 < t {
                let top = stack.pop().unwrap();
                result[top.1] = (i - top.1) as i32;
            }
            stack.push((t, i));
        }
        result
    }
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut fleets: i32 = 1;
        let mut ps: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
        ps.sort_unstable_by(|x, y| y.0.cmp(&x.0));
        let mut previous = (target - ps[0].0) as f64 / ps[0].1 as f64;
        for (p, s) in ps.into_iter().skip(1) {
            let current = (target - p) as f64 / s as f64;
            if previous < current {
                fleets += 1;
                previous = current;
            }
        }

        fleets
    }
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }

    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        if let Some(node) = root {
            stack.push(node.clone());
            root = Some(node);
        }

        while let Some(node) = stack.pop() {
            let mut node_ref = node.borrow_mut();
            let temp = node_ref.left.take();
            node_ref.left = node_ref.right.take();
            node_ref.right = temp;
            if let Some(l) = node_ref.left.clone() {
                stack.push(l);
            }
            if let Some(r) = node_ref.right.clone() {
                stack.push(r);
            }
        }

        root
    }

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
