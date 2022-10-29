use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::get_depth(&root) >= 0
    }

    fn get_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let depth_left = Self::get_depth(&r.borrow().left);
            let depth_right = Self::get_depth(&r.borrow().right);

            if depth_left == -1 || depth_right == -1 || (depth_left - depth_right).abs() > 1 {
                return -1;
            }

            depth_left.max(depth_right) + 1
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
        let root = tree!(
            3,
            tree!(9),
            tree!(20, tree!(15), tree!(7))
        );
        assert_eq!(Solution::is_balanced(root), true);

        let root = tree!(
            1,
            tree!(2, tree!(3, tree!(4), tree!(4)), tree!(3)),
            tree!(2)
        );
        assert_eq!(Solution::is_balanced(root), false);

        assert_eq!(Solution::is_balanced(None), true);
    }
}
