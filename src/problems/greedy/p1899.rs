pub mod first_attempt {
    pub struct Solution;
    impl Solution {
        pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
            // triplets.sort();
            let (a, b, c) = (target[0], target[1], target[2]);
            let mut check: Vec<usize> = Vec::with_capacity(triplets.len());
            for i in 0..triplets.len() {
                if target == triplets[i] {
                    return true;
                }
                let (x, y, z) = (triplets[i][0], triplets[i][1], triplets[i][2]);
                if (x == a || y == b || z == c) && (x <= a && y <= b && z <= c) {
                    check.push(i);
                }
            }

            for i in check.iter() {
                let i = *i;

                let mut current = triplets[i].clone();
                for j in check.iter() {
                    let j = *j;
                    if j == i {
                        continue;
                    }
                    // println!("{current:?} {:?}", triplets[j]);

                    current[0] = current[0].max(triplets[j][0]);
                    current[1] = current[1].max(triplets[j][1]);
                    current[2] = current[2].max(triplets[j][2]);
                    if current == target {
                        return true;
                    }
                }
            }

            false
        }
    }
}

pub mod second_attempt {
    pub struct Solution;
    impl Solution {
        pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
            let (a, b, c) = (target[0], target[1], target[2]);
            let mut current: [i32; 3usize] = [0, 0, 0];
            for i in 0..triplets.len() {
                if target == triplets[i] {
                    return true;
                }
                let (x, y, z) = (triplets[i][0], triplets[i][1], triplets[i][2]);
                let is_relevant: bool =
                    (x == a || y == b || z == c) && (x <= a && y <= b && z <= c);
                if !is_relevant {
                    continue;
                }

                current[0] = current[0].max(x);
                current[1] = current[1].max(y);
                current[2] = current[2].max(z);

                if current == *target {
                    return true;
                }
            }

            false
        }
    }
}
