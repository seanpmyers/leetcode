use std::{cell::RefCell, rc::Rc};

pub type SegmentNode = Rc<RefCell<SegmentTree>>;

#[derive(Debug)]
pub struct SegmentTree {
    pub sum: i32,
    pub left: Option<SegmentNode>,
    pub right: Option<SegmentNode>,
    pub left_index: usize,
    pub right_index: usize,
}

impl SegmentTree {
    pub fn build(numbers: &[i32], left_index: usize, right_index: usize) -> Self {
        let mut result = Self {
            sum: 0,
            left: None,
            right: None,
            left_index,
            right_index,
        };

        let middle: usize = right_index.midpoint(left_index);

        if left_index.abs_diff(right_index) == 0 {
            result.sum = numbers[left_index];
            return result;
        }

        result.left = Some(Rc::new(RefCell::new(Self::build(
            &numbers, left_index, middle,
        ))));

        result.right = Some(Rc::new(RefCell::new(Self::build(
            &numbers,
            middle + 1,
            right_index,
        ))));

        result.sum =
            Self::get_node_sum(result.left.as_ref()) + Self::get_node_sum(result.right.as_ref());

        result
    }

    fn get_node_sum(node: Option<&SegmentNode>) -> i32 {
        node.map_or(0, |n| n.borrow().sum)
    }

    pub fn update(&mut self, index: usize, value: i32) {
        if self.left_index == self.right_index {
            self.sum = value;
            return;
        }

        let middle: usize = self.left_index.midpoint(self.right_index);

        match index.cmp(&middle) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                if let Some(left) = &self.left {
                    left.borrow_mut().update(index, value);
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(right) = &self.right {
                    right.borrow_mut().update(index, value);
                }
            }
        };

        self.sum = Self::get_node_sum(self.left.as_ref()) + Self::get_node_sum(self.right.as_ref())
    }

    pub fn range_query(&self, left_index: usize, right_index: usize) -> i32 {
        if self.left_index == left_index && self.right_index == right_index {
            return self.sum;
        }

        let middle: usize = self.right_index.midpoint(self.left_index);

        if left_index > middle
            && let Some(right) = &self.right
        {
            return right.borrow().range_query(left_index, right_index);
        }

        if right_index <= middle
            && let Some(left) = &self.left
        {
            return left.borrow().range_query(left_index, right_index);
        }

        self.left
            .as_ref()
            .map_or(0, |n| n.borrow().range_query(left_index, middle))
            + self
                .right
                .as_ref()
                .map_or(0, |n| n.borrow().range_query(middle + 1, right_index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_and_query_entire_range_test() {
        let nums = vec![1, 3, 5, 7, 9, 11];
        let root = SegmentTree::build(&nums, 0, nums.len() - 1);

        // Sum of all elements: 1+3+5+7+9+11 = 36
        assert_eq!(root.range_query(0, 5), 36);
    }

    #[test]
    fn partial_range_queries_test() {
        let nums = vec![1, 3, 5, 7, 9, 11];
        let root = SegmentTree::build(&nums, 0, nums.len() - 1);

        assert_eq!(root.range_query(0, 2), 9); // 1 + 3 + 5
        assert_eq!(root.range_query(3, 5), 27); // 7 + 9 + 11
        assert_eq!(root.range_query(1, 4), 24); // 3 + 5 + 7 + 9
    }

    #[test]
    fn point_update_test() {
        let nums = vec![1, 3, 5, 7, 9, 11];
        let mut root = SegmentTree::build(&nums, 0, nums.len() - 1);

        root.update(2, 10);

        // New sum: 1 + 3 + 10 + 7 + 9 + 11 = 41
        assert_eq!(root.range_query(0, 5), 41);
        // Verify specific range affected by the update
        assert_eq!(root.range_query(0, 2), 14); // 1 + 3 + 10
    }

    #[test]
    fn single_element_tree_test() {
        let nums = vec![42];
        let mut root = SegmentTree::build(&nums, 0, 0);

        assert_eq!(root.range_query(0, 0), 42);

        root.update(0, 100);
        assert_eq!(root.range_query(0, 0), 100);
    }

    #[test]
    fn large_point_updates_test() {
        let nums = vec![1, 1, 1, 1];
        let mut root = SegmentTree::build(&nums, 0, 3);

        root.update(0, 2);
        root.update(1, 2);
        root.update(2, 2);
        root.update(3, 2);

        assert_eq!(root.range_query(0, 3), 8);
        assert_eq!(root.range_query(1, 2), 4);
    }
}
