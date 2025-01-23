pub struct Solution;
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashMap, HashSet};
        let mut row: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        let mut col: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] == 0 {
                    continue;
                }
                row.entry(r)
                    .and_modify(|x| x.push((r, c)))
                    .or_insert(vec![(r, c)]);
                col.entry(c)
                    .and_modify(|x| x.push((r, c)))
                    .or_insert(vec![(r, c)]);
            }
        }
        let mut result: HashSet<(usize, usize)> = HashSet::new();
        row.into_values().filter(|x| x.len() > 1).for_each(|x| {
            for comp in x.into_iter() {
                result.insert(comp);
            }
        });
        col.into_values().filter(|x| x.len() > 1).for_each(|x| {
            for comp in x.into_iter() {
                result.insert(comp);
            }
        });
        result.len() as i32
    }
}
