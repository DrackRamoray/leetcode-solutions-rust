use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem::swap;

struct Solution;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = vec![];
        let mut cur = root.clone();
        let mut left = None;
        let mut right = None;
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        while !stack.is_empty() || cur.is_some() {
            while let Some(node) = cur {
                cur = node.borrow_mut().left.clone();
                stack.push(node);
            }

            if let Some(node) = stack.pop() {
                if let Some(p) = prev {
                    if p.borrow_mut().val > node.borrow_mut().val {
                        right = Some(node.clone());

                        if left.is_none() {
                            left = Some(p);
                        } else {
                            break;
                        }
                    }
                }

                prev = Some(node.clone());
                cur = node.borrow_mut().right.clone();
            }
        }

        swap(&mut left.unwrap().borrow_mut().val, &mut right.unwrap().borrow_mut().val)
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
        let mut root = tree!(1, tree!(3, None, tree!(2)), None);
        let ans = tree!(3, tree!(1, None, tree!(2)), None);
        Solution::recover_tree(&mut root);
        assert_eq!(root, ans);

        let mut root = tree!(3, tree!(1), tree!(4, tree!(2), None));
        let ans = tree!(2, tree!(1), tree!(4, tree!(3), None));
        Solution::recover_tree(&mut root);
        assert_eq!(root, ans);
    }
}
