use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            node.right = Self::invert_tree(left);
            node.left = Self::invert_tree(right);
        }

        root
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
        let root = tree!(4, tree!(2, tree!(1), tree!(3)), tree!(7, tree!(6), tree!(9)));
        let ans = tree!(4, tree!(7, tree!(9), tree!(6)), tree!(2, tree!(3), tree!(1)));
        assert_eq!(Solution::invert_tree(root), ans);
    }
}
