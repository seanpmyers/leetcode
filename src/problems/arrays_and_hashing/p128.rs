// #arrays #hashing

pub mod hashing {
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
    }
}

pub mod disjoint_set_union {
    pub struct Solution;
    use std::collections::HashMap;
    impl Solution {
        pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
            let mut parents: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
            let mut ranks: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
            if nums.len() <= 1 {
                return nums.len() as i32;
            }

            let mut result: i32 = 1;

            for n in &nums {
                if parents.get(n).is_some() {
                    continue;
                }
                parents.insert(*n, *n);
                ranks.insert(*n, 1);
            }

            for &n in &nums {
                if parents.get(&(n - 1)).is_none() {
                    continue;
                }
                let rank: i32 = Self::union(n, n - 1, &mut parents, &mut ranks);
                result = result.max(rank);
            }

            result
        }

        pub fn find(n: i32, parents: &mut HashMap<i32, i32>) -> i32 {
            let mut result: i32 = n;
            while let Some(parent) = parents.get(&result)
                && *parent != result
            {
                let Some(&grandparent) = parents.get(&parent) else {
                    continue;
                };
                parents.entry(result).and_modify(|p| *p = grandparent);
                result = grandparent;
            }

            result
        }

        pub fn union(
            x: i32,
            y: i32,
            parents: &mut HashMap<i32, i32>,
            ranks: &mut HashMap<i32, i32>,
        ) -> i32 {
            let x: i32 = Self::find(x, parents);
            let y: i32 = Self::find(y, parents);

            let x_rank: i32 = *ranks.get(&x).unwrap();
            let y_rank: i32 = *ranks.get(&y).unwrap();

            if x == y {
                return x_rank;
            }

            let rank = x_rank + y_rank;

            match x_rank < y_rank {
                true => {
                    parents.insert(x, y);
                    ranks.insert(y, rank);
                }
                false => {
                    parents.insert(y, x);
                    ranks.insert(x, rank);
                }
            }

            rank
        }
    }
}
