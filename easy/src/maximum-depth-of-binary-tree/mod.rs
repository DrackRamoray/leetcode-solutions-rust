use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::depth(&root)
    }

    pub fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let dl = Self::depth(&r.borrow().left);
            let dr = Self::depth(&r.borrow().right);

            dl.max(dr) + 1
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
        assert_eq!(Solution::max_depth(root), 3);
    }
}
