use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&nums, 0, nums.len() - 1)
    }

    fn helper (nums: &Vec<i32>, left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }

        let mid = left + (right - left) / 2;

        let mut root = TreeNode::new(nums[mid]);

        if mid.wrapping_sub(1) < nums.len() {
            root.left = Self::helper(nums, left, mid - 1);
        }

        root.right = Self::helper(nums, mid + 1, right);

        Some(Rc::new(RefCell::new(root)))
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
        let nums = vec![-10,-3,0,5,9];
        let root = tree!(
            0,
            tree!(-10, None,  tree!(-3)),
            tree!(5, None, tree!(9))
        );
        assert_eq!(Solution::sorted_array_to_bst(nums), root);

        let nums = vec![1,3];
        let root = tree!(
            1,
            None,
            tree!(3)
        );
        assert_eq!(Solution::sorted_array_to_bst(nums), root);
    }
}
