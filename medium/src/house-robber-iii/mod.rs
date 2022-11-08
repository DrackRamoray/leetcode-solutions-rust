use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (selected, skipped) = Self::dfs(&root);

        selected.max(skipped)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(r) = root {
            let (left_selected, left_skipped) = Self::dfs(&r.borrow().left);
            let (right_selected, right_skipped) = Self::dfs(&r.borrow().right);

            let selected = r.borrow().val + left_skipped + right_skipped;
            let skipped = left_selected.max(left_skipped) + right_selected.max(right_skipped);

            (selected, skipped)
        } else {
            (0, 0)
        }
    }
}

#[test]
fn it_works() {
    use assist::tree;

    assert_eq!(Solution::rob(tree!(3, tree!(2, None, tree!(3)), tree!(3, None, tree!(1)))), 7);
    assert_eq!(Solution::rob(tree!(3, tree!(4, tree!(1), tree!(3)), tree!(5, None, tree!(1)))), 9);
}
