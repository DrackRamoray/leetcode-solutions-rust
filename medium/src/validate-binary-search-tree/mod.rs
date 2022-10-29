use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn is_valid_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = vec![];
        let mut prev = i64::MIN;

        while stack.len() > 0 || root.is_some() {
            while let Some(r) = root {
                root = r.borrow_mut().left.take();
                stack.push(r);
            }

            if let Some(r) = stack.pop() {
                if (r.borrow().val as i64) <= prev {
                    return false;
                }

                prev = r.borrow().val as i64;
                root = r.borrow_mut().right.take();
            }
        }

        return true;
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
        let root = tree!(2, tree!(1), tree!(3));
        let ans = true;
        assert_eq!(Solution::is_valid_bst(root), ans);
        let root = tree!(5, tree!(1), tree!(4, tree!(3), tree!(6)));
        let ans = false;
        assert_eq!(Solution::is_valid_bst(root), ans);
    }
}
