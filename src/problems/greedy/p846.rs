pub mod first_attempt {
    pub struct Solution;
    use std::collections::VecDeque;
    impl Solution {
        pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
            let n: usize = group_size as usize;
            if hand.len() % n != 0 {
                return false;
            }

            if n == 1 {
                return true;
            }
            hand.sort();
            let mut hand: VecDeque<i32> = VecDeque::from(hand);
            let mut count: usize = 0;

            while !hand.is_empty() {
                let mut previous: i32 = hand.pop_front().unwrap();
                let mut hold: Vec<i32> = vec![];
                count += 1;
                while let Some(current) = hand.pop_front() {
                    if current == previous {
                        hold.push(current);
                        continue;
                    }
                    if current != previous + 1 {
                        return false;
                    }
                    count += 1;
                    previous = current;
                    if count >= n {
                        break;
                    }
                }
                if count < n {
                    return false;
                }
                count = 0;
                while let Some(held) = hold.pop() {
                    hand.push_front(held);
                }
            }

            true
        }
    }
}
