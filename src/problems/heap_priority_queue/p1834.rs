pub mod first {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    impl Solution {
        pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
            let tasks: &[Vec<i32>] = &tasks[..tasks.len()];
            let mut task_heap: BinaryHeap<(Reverse<i32>, Reverse<usize>)> =
                BinaryHeap::with_capacity(tasks.len());
            let mut heap: BinaryHeap<(Reverse<i32>, Reverse<usize>)> =
                BinaryHeap::with_capacity(tasks.len());
            for (i, task) in tasks.iter().enumerate() {
                task_heap.push((Reverse(task[0]), Reverse(i)));
            }
            let mut result: Vec<i32> = Vec::with_capacity(tasks.len());

            let mut time: i32 = 0;
            while !task_heap.is_empty() || !heap.is_empty() {
                while task_heap.peek().is_some_and(|t| time >= t.0.0) {
                    let (Reverse(_time), Reverse(i)) = task_heap.pop().unwrap();
                    heap.push((Reverse(tasks[i][1]), Reverse(i)));
                }

                if let Some((Reverse(speed), Reverse(i))) = heap.pop() {
                    time += speed;
                    result.push(i as i32);
                    continue;
                }

                if heap.is_empty() && task_heap.peek().is_some_and(|t| time < t.0.0) {
                    time = task_heap.peek().cloned().unwrap().0.0;
                }
            }

            result
        }
    }
}
