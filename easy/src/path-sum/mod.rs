use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::traverse(&root, target_sum, 0)
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32, sum: i32) -> bool {
        if let Some(r) = root {
            let sum = sum + r.borrow().val;
            if r.borrow().left.is_none() && r.borrow().right.is_none() {
                return sum == target_sum;
            }

            Self::traverse(&r.borrow().left, target_sum, sum) || Self::traverse(&r.borrow().right, target_sum, sum)
        } else {
            false
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
        let root = tree!(
            5,
            tree!(4, tree!(11, tree!(7), tree!(2)), None),
            tree!(8, tree!(13), tree!(4, None, tree!(1)))
        );
        assert_eq!(Solution::has_path_sum(root, 22), true);

        let root = tree!(
            1,
            tree!(2),
            tree!(3)
        );
        assert_eq!(Solution::has_path_sum(root, 5), false);

        assert_eq!(Solution::has_path_sum(None, 0), false);
    }
}
