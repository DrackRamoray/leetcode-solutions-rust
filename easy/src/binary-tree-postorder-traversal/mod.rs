use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        let mut node = root;
        let mut prev = None;

        while node.is_some() || stack.len() > 0 {
            while let Some(n) = node {
                node = n.borrow_mut().left.take();
                stack.push(n);
            }

            if let Some(n) = stack.pop() {
                if n.borrow().right.is_none() || n.borrow().right == prev {
                    ans.push(n.borrow().val);
                    prev = Some(n);
                    node = None;
                } else {
                    node = n.borrow_mut().right.take();
                    stack.push(n);
                }
            }
        }

        ans
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
        let root = tree!(1, None, tree!(2, tree!(3), None));
        let ans = vec![3,2,1];
        assert_eq!(Solution::postorder_traversal(root), ans);

        let root = None;
        let ans = vec![] as Vec<i32>;
        assert_eq!(Solution::postorder_traversal(root), ans);

        let root = tree!(1);
        let ans = vec![1];
        assert_eq!(Solution::postorder_traversal(root), ans);
    }
}
