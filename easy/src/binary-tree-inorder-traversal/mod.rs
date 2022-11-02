use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        let mut node = root;

        while node.is_some() || stack.len() > 0 {
            while let Some(n) = node {
                node = n.borrow_mut().left.take();
                stack.push(n);
            }

            if let Some(n) = stack.pop() {
                ans.push(n.borrow().val);
                node = n.borrow_mut().right.take();
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use assist::{TreeNode, tree};
    use super::Solution;

    #[test]
    fn it_works() {
        let root = tree!(1,None,tree!(2,tree!(3),None));
        let ans = vec![1, 3, 2];
        assert_eq!(Solution::inorder_traversal(root), ans);

        let root = None;
        let ans = vec![] as Vec<i32>;
        assert_eq!(Solution::inorder_traversal(root), ans);

        let root = tree!(1);
        let ans = vec![1];
        assert_eq!(Solution::inorder_traversal(root), ans);
    }
}
