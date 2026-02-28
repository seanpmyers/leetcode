pub mod first_attempt {
    pub struct Solution;
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;
    impl Solution {
        pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
            let mut map: HashMap<i32, Rc<RefCell<HashSet<i32>>>> =
                HashMap::with_capacity(edges.len());
            let mut unique: HashSet<i32> = HashSet::new();
            for e in edges.iter() {
                let x: i32 = e[0];
                let y: i32 = e[1];
                unique.insert(x);
                unique.insert(y);
                match (map.get(&x).cloned(), map.get(&y).cloned()) {
                    (Some(xl), Some(yl)) => {
                        if Rc::ptr_eq(&xl, &yl) {
                            continue;
                        }
                        xl.borrow_mut().insert(x);
                        xl.borrow_mut().insert(y);
                        for i in yl.borrow().iter() {
                            xl.borrow_mut().insert(*i);
                            map.entry(*i).and_modify(|list| *list = xl.clone());
                        }
                    }
                    (Some(z), None) => {
                        z.borrow_mut().insert(x);
                        z.borrow_mut().insert(y);
                        map.insert(y, z.clone());
                    }
                    (None, Some(z)) => {
                        z.borrow_mut().insert(x);
                        z.borrow_mut().insert(y);
                        map.insert(x, z.clone());
                    }
                    (None, None) => {
                        let list: Rc<RefCell<HashSet<i32>>> =
                            Rc::new(RefCell::new(HashSet::from([x, y])));
                        map.insert(x, list.clone());
                        map.insert(y, list.clone());
                    }
                };
            }
            let mut result: HashSet<Vec<i32>> = HashSet::new();
            for v in map.values() {
                let vals = v.borrow().clone().into_iter().collect::<Vec<i32>>();
                if result.contains(&vals) {
                    // println!("same");
                    continue;
                }
                // println!("diff {:?}", vals);
                result.insert(vals);
            }

            if (unique.len() as i32) < n {
                result.len() as i32 + (n - unique.len() as i32)
            } else {
                result.len() as i32
            }
        }
    }
}

pub mod disjoint_set_union {
    pub struct Solution;
    impl Solution {
        pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
            let mut result: i32 = n;
            let n = n as usize;
            let mut parents: Vec<usize> = (0..n).collect::<Vec<usize>>();
            let mut ranks: Vec<usize> = vec![1usize; n];

            for edge in &edges {
                let x = edge[0];
                let y = edge[1];

                if Self::union(x as usize, y as usize, &mut parents, &mut ranks) {
                    result -= 1;
                }
            }

            result
        }

        pub fn find(parents: &mut Vec<usize>, n: usize) -> usize {
            let mut parent: usize = n;
            while parents[parent] != parent {
                parents[parent] = parents[parents[parent]];
                parent = parents[parent];
            }

            parent
        }

        pub fn union(x: usize, y: usize, parents: &mut Vec<usize>, ranks: &mut Vec<usize>) -> bool {
            let xp: usize = Self::find(parents, x);
            let yp: usize = Self::find(parents, y);
            if xp == yp {
                return false;
            }

            match ranks[xp].cmp(&ranks[yp]) {
                std::cmp::Ordering::Greater => parents[yp] = xp,
                std::cmp::Ordering::Less => parents[xp] = yp,
                std::cmp::Ordering::Equal => {
                    parents[xp] = yp;
                    ranks[yp] = ranks[yp] + 1;
                }
            }

            true
        }
    }
}
