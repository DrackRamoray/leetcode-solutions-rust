use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        Self::is_unival_subtrees(&root, i32::MAX, &mut count);
        count
    }

    fn is_unival_subtrees(root: &Option<Rc<RefCell<TreeNode>>>, val: i32, count: &mut i32) -> bool {
        if let Some(r) = root {
            let b1 = Self::is_unival_subtrees(&r.borrow().left, r.borrow().val, count);
            let b2 = Self::is_unival_subtrees(&r.borrow().right, r.borrow().val, count);

            if !b1 || !b2 {
                return false;
            }

            *count += 1;

            r.borrow().val == val
        } else {
            true
        }
    }
}

#[test]
fn it_works() {
    use assist::tree;
    let root = tree!(5, tree!(1, tree!(5), tree!(5)), tree!(5, None, tree!(5)));
    assert_eq!(Solution::count_unival_subtrees(root), 4);
}
