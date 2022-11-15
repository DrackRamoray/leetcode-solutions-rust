use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum(&root)
    }

    fn sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut total = 0;
        if let Some(r) = root {
            if let Some(left) = &r.borrow().left {
                if left.borrow().left.is_none() && left.borrow().right.is_none() {
                    total += left.borrow().val;
                }
            }

            total += Self::sum(&r.borrow().left);
            total += Self::sum(&r.borrow().right);
        }

        total
    }
}

#[test]
fn it_works() {
    use assist::tree;
    assert_eq!(Solution::sum_of_left_leaves(tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)))), 24);
    assert_eq!(Solution::sum_of_left_leaves(tree!(1)), 0);
}
