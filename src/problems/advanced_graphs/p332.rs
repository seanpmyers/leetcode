pub mod hierholzer {
    pub struct Solution;
    use std::collections::HashMap;
    impl Solution {
        pub const JFK: &'static str = "JFK";
        pub fn find_itinerary(mut tickets: Vec<Vec<String>>) -> Vec<String> {
            let mut adj: HashMap<String, Vec<String>> = HashMap::with_capacity(tickets.len());
            let mut result: Vec<String> = vec![];
            tickets.sort_by(|a, b| b.cmp(&a));

            for ticket in tickets.into_iter() {
                let from: String = ticket[0].clone();
                let to: String = ticket[1].clone();

                let entry = adj.entry(from).or_default();
                entry.push(to);
            }

            let mut queue: Vec<String> = vec![Self::JFK.to_string()];

            while let Some(location) = queue.pop() {
                let entry = adj.entry(location.clone()).or_default();
                let Some(next) = entry.pop() else {
                    result.push(location);
                    continue;
                };
                queue.push(location);
                queue.push(next);
            }

            result.reverse();
            result
        }
    }
}
