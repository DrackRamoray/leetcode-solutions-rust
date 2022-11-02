use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut k = k;
        let mut node = root;
        let mut stack = vec![];

        while node.is_some() || stack.len() > 0 {
            while let Some(n) = node {
                node = n.borrow_mut().left.take();
                stack.push(n);
            }

            if let Some(n) = stack.pop() {
                k -= 1;

                if k == 0 {
                    node = Some(n);
                    break;
                }

                node = n.borrow_mut().right.take();
            }
        }

        if let Some(n) = node {
            n.borrow().val
        } else {
            -1
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
        let root = tree!(3, tree!(1, None, tree!(2)), tree!(4));
        let k = 1;
        let ans = 1;
        assert_eq!(Solution::kth_smallest(root, k), ans);

        let root = tree!(5, tree!(3, tree!(2, tree!(1), None), tree!(4)), tree!(6));
        let k = 3;
        let ans = 3;
        assert_eq!(Solution::kth_smallest(root, k), ans);
    }
}
