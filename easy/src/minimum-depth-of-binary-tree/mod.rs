use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::depth(&root)
    }

    fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                match (&node.left, &node.right) {
                    (None, None) => 1,
                    (Some(_), None) => Self::depth(&node.left) + 1,
                    (None, Some(_)) => Self::depth(&node.right) + 1,
                    (Some(_), Some(_)) => {
                        Self::depth(&node.left).min(Self::depth(&node.right)) + 1
                    }
                }
            }
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
        assert_eq!(Solution::min_depth(root), 2);

        let root = tree!(
            2,
            None,
            tree!(3, None, tree!(4, None, tree!(5, None, tree!(6))))
        );
        assert_eq!(Solution::min_depth(root), 5);
    }
}
