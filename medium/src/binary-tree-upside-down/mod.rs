use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn upside_down_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::pre_order(root, None, None)
    }

    fn pre_order(root: Option<Rc<RefCell<TreeNode>>>, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            let left_sub_tree = r.borrow_mut().left.take();
            let right_sub_tree = r.borrow_mut().right.take();
            r.borrow_mut().left = left;
            r.borrow_mut().right = right;

            Self::pre_order(left_sub_tree, right_sub_tree, Some(r))
        } else {
            right
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
        let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3));
        let ans = tree!(4, tree!(5), tree!(2, tree!(3), tree!(1)));
        assert_eq!(Solution::upside_down_binary_tree(root), ans);

        let root = None;
        let ans = None;
        assert_eq!(Solution::upside_down_binary_tree(root), ans);

        let root = tree!(1);
        let ans = tree!(1);
        assert_eq!(Solution::upside_down_binary_tree(root), ans);
    }
}
