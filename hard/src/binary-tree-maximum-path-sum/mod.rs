use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;

        Self::max_gain(&root, &mut max_sum);

        max_sum
    }

    fn max_gain (node: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        if let Some(n) = node {
            let left_gain = 0.max(Self::max_gain(&n.borrow().left, max_sum));
            let right_gain = 0.max(Self::max_gain(&n.borrow().right, max_sum));

            *max_sum = (*max_sum).max(n.borrow().val + left_gain + right_gain);

            n.borrow().val + left_gain.max(right_gain)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use assist::{TreeNode, tree};
    use std::rc::Rc;
    use std::cell::RefCell;
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_path_sum(tree!(1, tree!(2), tree!(3))), 6);
        assert_eq!(Solution::max_path_sum(tree!(-10, tree!(9), tree!(20, tree!(15), tree!(7)))), 42);
    }
}
