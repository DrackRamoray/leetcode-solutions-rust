use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if p == root || q == root {
            return root;
        }

        if let Some(r) = root {
            let left = Self::lowest_common_ancestor(r.borrow_mut().left.take(), p.clone(), q.clone());
            let right = Self::lowest_common_ancestor(r.borrow_mut().right.take(), p, q);

            if left.is_none() {
                return right;
            }

            if right.is_none() {
                return left;
            }

            Some(r)
        } else {
            None
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
        let p = tree!(5, tree!(6), tree!(2, tree!(7), tree!(4)));
        let q = tree!(1, tree!(0), tree!(8));
        let root = tree!(3, p.clone(), q.clone());

        assert_eq!(Solution::lowest_common_ancestor(root.clone(), p.clone(), q), root.clone());

        let q = tree!(4);
        let p = tree!(5, tree!(6), tree!(2, tree!(7), q.clone()));
        let root = tree!(3, p.clone(), tree!(1, tree!(0), tree!(8)));
        assert_eq!(Solution::lowest_common_ancestor(root, p.clone(), q), p.clone());
    }
}
