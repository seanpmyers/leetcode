pub mod math {
    pub struct Solution;
    impl Solution {
        pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
            let mut count: [i32; 26usize] = [0i32; 26usize];

            for task in tasks.iter() {
                count[(*task as u8 - b'A') as usize] += 1;
            }

            let max: i32 = *count.iter().max().unwrap_or(&0);
            let max_count: i32 = count.iter().filter(|x| **x == max).count() as i32;

            let time = max.saturating_sub(1) * n.saturating_add(1) + max_count;
            (tasks.len() as i32).max(time)
        }
    }
}
pub mod better {
    pub struct Solution;
    use std::collections::{BinaryHeap, HashMap, VecDeque};
    impl Solution {
        pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
            let mut result: i32 = 0;
            let mut map: HashMap<char, i32> = HashMap::with_capacity(26usize);
            let mut heap: BinaryHeap<(i32, char, i32)> = BinaryHeap::with_capacity(26usize);
            let mut cooldown: VecDeque<(i32, char, i32)> = VecDeque::with_capacity(26usize);

            for t in tasks.iter() {
                map.entry(*t)
                    .and_modify(|x| *x = x.saturating_add(1))
                    .or_insert(1i32);
            }

            for (k, v) in map.iter() {
                heap.push((*v, *k, 0i32.saturating_sub(n)));
            }

            while !heap.is_empty() || !cooldown.is_empty() {
                result = result.saturating_add(1);
                if cooldown
                    .front()
                    .is_some_and(|x| result.saturating_sub(1).saturating_sub(x.2) >= n)
                    && let Some((count, letter, last_cycle)) = cooldown.pop_front()
                {
                    heap.push((count, letter, last_cycle));
                }
                let Some((count, letter, last_cycle)) = heap.pop() else {
                    continue;
                };
                if result.saturating_sub(1).saturating_sub(last_cycle) >= n
                    && count.saturating_sub(1) > 0
                {
                    cooldown.push_back((count.saturating_sub(1), letter, result));
                }
            }

            result
        }
    }
}

pub mod linear_time {
    pub struct Solution;
    use std::collections::{BinaryHeap, HashMap};
    impl Solution {
        pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
            let mut result: i32 = 0;
            let mut map: HashMap<char, i32> = HashMap::with_capacity(26usize);
            let mut heap: BinaryHeap<(i32, char, i32)> = BinaryHeap::with_capacity(26usize);

            for t in tasks.iter() {
                map.entry(*t)
                    .and_modify(|x| *x = x.saturating_add(1))
                    .or_insert(1i32);
            }

            for (k, v) in map.iter() {
                heap.push((*v, *k, 0i32.saturating_sub(n)));
            }

            while let Some((count, letter, last_cycle)) = heap.pop() {
                result = result.saturating_add(1);
                if result.saturating_sub(1).saturating_sub(last_cycle) >= n {
                    if count.saturating_sub(1) > 0 {
                        heap.push((count.saturating_sub(1), letter, result));
                    }
                    continue;
                }

                let mut requeue: Vec<(i32, char, i32)> = Vec::with_capacity(26);
                requeue.push((count, letter, last_cycle));

                while let Some((inner_count, inner_letter, inner_last_cycle)) = heap.pop() {
                    if result.saturating_sub(1).saturating_sub(inner_last_cycle) >= n {
                        if inner_count.saturating_sub(1) > 0 {
                            requeue.push((inner_count.saturating_sub(1), inner_letter, result));
                        }
                        break;
                    }
                    requeue.push((inner_count, inner_letter, inner_last_cycle))
                }

                for task in requeue.into_iter() {
                    heap.push((task.0, task.1, task.2));
                }
            }

            result
        }
    }
}
