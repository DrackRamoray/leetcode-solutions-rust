use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut prev = None;
        Self::postorder(root.take(), &mut prev);
        *root = prev;
    }

    fn postorder(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            let left = r.borrow_mut().left.take();
            let right = r.borrow_mut().right.take();
            Self::postorder(right, prev);
            Self::postorder(left, prev);
            r.borrow_mut().right = prev.take();
            *prev = Some(r);
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
        let mut root = tree!(1, tree!(2, tree!(3), tree!(4)), tree!(5, None, tree!(6)));
        let ans = tree!(1, None, tree!(2, None, tree!(3, None, tree!(4, None, tree!(5, None, tree!(6))))));
        Solution::flatten(&mut root);
        assert_eq!(root, ans);

        let mut root = None;
        let ans = None;
        Solution::flatten(&mut root);
        assert_eq!(root, ans);

        let mut root = tree!(1);
        let ans = tree!(1);
        Solution::flatten(&mut root);
        assert_eq!(root, ans);
    }
}
