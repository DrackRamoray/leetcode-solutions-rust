use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn inorder_successor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![];
        let mut node = root;
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        while stack.len() > 0 || node.is_some() {
            while let Some(n) = node  {
                node = n.borrow_mut().left.take();
                stack.push(n);
            }

            if let Some(n) = stack.pop() {
                node = n.borrow_mut().right.take();

                if p == prev {
                    return Some(n);
                }

                prev = Some(n);
            }
        }

        None
    }
}

#[test]
fn it_works() {
    use assist::tree;

    let root = tree!(2, tree!(1), tree!(3));
    let p = tree!(1);
    let ans = tree!(2);
    assert_eq!(Solution::inorder_successor(root, p), ans);

    let root = tree!(5, tree!(3, tree!(3, tree!(2, tree!(1), None), None), tree!(4)), tree!(6));
    let p = tree!(6);
    let ans: Option<Rc<RefCell<TreeNode>>> = None;
    assert_eq!(Solution::inorder_successor(root, p), ans);
}
