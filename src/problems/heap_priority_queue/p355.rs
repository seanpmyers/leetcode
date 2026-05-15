#[allow(dead_code)]
pub mod heap {
    use std::collections::{BinaryHeap, HashMap, HashSet};
    struct Twitter {
        pub tweets: HashMap<i32, Vec<(i32, i32, i32)>>,
        pub following: HashMap<i32, HashSet<i32>>,
        pub time: i32,
    }

    impl Twitter {
        fn new() -> Self {
            Self {
                tweets: HashMap::new(),
                following: HashMap::new(),
                time: 0i32,
            }
        }

        fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
            self.time += 1;
            self.tweets
                .entry(user_id)
                .or_default()
                .push((self.time, user_id, tweet_id));
        }

        fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
            let mut heap: BinaryHeap<(i32, i32, i32, usize)> = BinaryHeap::new();

            let mut result: Vec<i32> = Vec::with_capacity(10);

            if let Some(latest) = self.get_latest_tweet(user_id) {
                heap.push(latest);
            }

            if let Some(following) = self.following.get(&user_id) {
                for f in following.iter() {
                    let Some(latest) = self.get_latest_tweet(*f) else {
                        continue;
                    };
                    heap.push(latest);
                }
            }

            while let Some((_time, user, tweet, index)) = heap.pop() {
                result.push(tweet);
                if result.len() == 10 {
                    return result;
                }

                let Some(next_index) = index.checked_sub(1) else {
                    continue;
                };
                let (t, u, tw) = self.tweets[&user][next_index];
                heap.push((t, u, tw, next_index));
            }

            result
        }

        fn get_latest_tweet(&self, user_id: i32) -> Option<(i32, i32, i32, usize)> {
            let Some(tweets) = self.tweets.get(&user_id) else {
                return None;
            };
            let index: usize = tweets.len() - 1;
            tweets
                .last()
                .map(|(time, user, tweet)| (*time, *user, *tweet, index))
        }

        fn follow(&mut self, follower_id: i32, followee_id: i32) {
            if follower_id == followee_id {
                return; // a user cannot follow themself.
            }

            self.following
                .entry(follower_id)
                .or_default()
                .insert(followee_id);
        }

        fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
            if follower_id == followee_id {
                return;
            }
            self.following
                .entry(follower_id)
                .or_default()
                .remove(&followee_id);
        }
    }
}
