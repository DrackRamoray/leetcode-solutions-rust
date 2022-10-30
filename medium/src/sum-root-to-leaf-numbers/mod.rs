use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(root, 0)
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if let Some(r) = root {
            let ans = sum * 10 + r.borrow().val;

            if r.borrow().left.is_none() && r.borrow().right.is_none() {
                ans
            } else {
                Self::helper(r.borrow().left.clone(), ans) + Self::helper(r.borrow().right.clone(), ans)
            }
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
        let root = tree!(1, tree!(2), tree!(3));
        assert_eq!(Solution::sum_numbers(root), 25);

        let root = tree!(4, tree!(9, tree!(5), tree!(1)), tree!(0));
        assert_eq!(Solution::sum_numbers(root), 1026);
    }
}
