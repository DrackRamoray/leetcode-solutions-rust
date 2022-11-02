use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() == 0 {
            return None;
        }

        let n = postorder.len();
        let val = postorder[n - 1];
        let index = inorder.iter().position(|&x| x == val).unwrap();

        let mut root = TreeNode::new(val);

        if index > 0 {
            root.left = Self::build_tree(inorder[0..index].to_vec(), postorder[0..index].to_vec());
        }

        if index < n - 1 {
            root.right = Self::build_tree(inorder[index+1..].to_vec(), postorder[index..n-1].to_vec());
        }

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
        let inorder = vec![9,3,15,20,7];
        let postorder = vec![9,15,7,20,3];
        let root = tree!(
            3,
            tree!(9),
            tree!(20, tree!(15), tree!(7))
        );
        assert_eq!(Solution::build_tree(inorder, postorder), root);

        let inorder = vec![-1];
        let postorder = vec![-1];
        let root = tree!(-1);
        assert_eq!(Solution::build_tree(inorder, postorder), root);
    }
}
