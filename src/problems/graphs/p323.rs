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
