pub mod disjoint_set_union {
    use std::collections::HashMap;

    pub struct Dsu {
        pub parents: Vec<usize>,
        pub ranks: Vec<usize>,
    }

    impl Dsu {
        pub fn new(n: usize) -> Self {
            Self {
                parents: (0..n).collect(),
                ranks: vec![1; n],
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            let mut result: usize = x;
            while result != self.parents[result] {
                self.parents[result] = self.parents[self.parents[result]];
                result = self.parents[result];
            }
            result
        }

        pub fn union(&mut self, x: usize, y: usize) {
            let x: usize = self.find(x);
            let y: usize = self.find(y);

            if x == y {
                return;
            }

            if self.ranks[x] > self.ranks[y] {
                self.ranks[x] += self.ranks[y];
                self.parents[y] = x;
                return;
            }

            self.ranks[y] += self.ranks[x];
            self.parents[x] = y;
        }
    }

    pub struct Solution;
    impl Solution {
        pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
            let mut dsu: Dsu = Dsu::new(accounts.len());
            let mut email_map: HashMap<&str, usize> = HashMap::with_capacity(accounts.len());

            for i in 0..accounts.len() {
                for email in accounts[i].iter().skip(1) {
                    let Some(&parent) = email_map.get(&email.as_str()) else {
                        email_map.insert(email.as_str(), i);
                        continue;
                    };

                    dsu.union(i, parent);
                }
            }

            let mut map: HashMap<usize, Vec<String>> = HashMap::new();

            for (email, parent) in email_map.into_iter() {
                let root: usize = dsu.find(parent);
                map.entry(root).or_default().push(email.to_string());
            }

            map.into_iter()
                .map(|(i, mut list)| {
                    list.sort();
                    let mut result: Vec<String> = vec![accounts[i][0].clone()];
                    result.append(&mut list);
                    result
                })
                .collect::<Vec<Vec<String>>>()
        }
    }
}
