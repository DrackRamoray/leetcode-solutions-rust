use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::post_order(&root, &mut ans);
        ans
    }

    fn post_order(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> Option<(i32, i32, i32)> {
        if let Some(r) = root {
            let mut cnt = 1;
            let mut min = r.borrow().val;
            let mut max = r.borrow().val;

            let left = Self::post_order(&r.borrow().left, ans);
            let right = Self::post_order(&r.borrow().right, ans);

            if let Some(left) = left {
                if left.2 < r.borrow().val {
                    cnt += left.0;
                    min = min.min(left.1);
                } else {
                    return None;
                }
            } else {
                return None;
            }

            if let Some(right) = right {
                if r.borrow().val < right.1 {
                    cnt += right.0;
                    max = max.max(right.2);
                } else {
                    return None;
                }
            } else {
                return None;
            }

            *ans = cnt.max(*ans);

            Some((cnt, min, max))
        } else {
            Some((0, i32::MAX, i32::MIN))
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
        assert_eq!(Solution::largest_bst_subtree(tree!(10, tree!(5, tree!(1), tree!(8)), tree!(15, None, tree!(7)))), 3);
        assert_eq!(Solution::largest_bst_subtree(tree!(1, tree!(3, tree!(4), None), tree!(2, None, tree!(5)))), 2);
        assert_eq!(Solution::largest_bst_subtree(tree!(4, tree!(2, tree!(2, tree!(2, tree!(2), None), None), tree!(3)), tree!(7, tree!(5), None))), 2);
    }
}
