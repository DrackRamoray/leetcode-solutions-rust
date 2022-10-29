use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p_node), Some(q_node)) => {
                if p_node.borrow().val != q_node.borrow().val {
                    return false;
                }

                let mut a = p_node.borrow_mut();
                let mut b = q_node.borrow_mut();

                Self::is_same_tree(a.left.take(), b.left.take()) && Self::is_same_tree(a.right.take(), b.right.take())
            },
            _ => false,
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
        let q = tree!(
            1,
            tree!(2, tree!(1), tree!(1)),
            tree!(2, tree!(1), tree!(1))
        );
        let p = tree!(
            1,
            tree!(2, tree!(1), tree!(1)),
            tree!(2, tree!(1), tree!(1))
        );
        assert_eq!(Solution::is_same_tree(p, q), true);

        let q = tree!(
            1,
            tree!(2),
            tree!(3)
        );
        let p = tree!(
            1,
            tree!(2),
            tree!(3)
        );
        assert_eq!(Solution::is_same_tree(p, q), true);

        let q = tree!(
            1,
            tree!(2),
            None
        );
        let p = tree!(
            1,
            None,
            tree!(2)
        );
        assert_eq!(Solution::is_same_tree(p, q), false);

        let q = tree!(
            1,
            tree!(2),
            tree!(1)
        );
        let p = tree!(
            1,
            tree!(1),
            tree!(2)
        );
        assert_eq!(Solution::is_same_tree(p, q), false);
    }
}
