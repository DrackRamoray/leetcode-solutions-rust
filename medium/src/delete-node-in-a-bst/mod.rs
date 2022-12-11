use std::rc::Rc;
use std::cell::RefCell;
use assist::TreeNode;

pub struct Solution;

impl Solution {
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            if r.borrow().val > key {
                let left = Self::delete_node(r.borrow_mut().left.take(), key);
                r.borrow_mut().left = left;
                return Some(r);
            }

            if r.borrow().val < key {
                let right = Self::delete_node(r.borrow_mut().right.take(), key);
                r.borrow_mut().right = right;
                return Some(r);
            }

            if r.borrow().val == key {
                if r.borrow().left.is_none() && r.borrow().right.is_none() {
                    return None;
                }

                if r.borrow().left.is_none() {
                    return r.borrow_mut().right.take();
                }

                if r.borrow().right.is_none() {
                    return r.borrow_mut().left.take();
                }

                let successor = Self::find_node(&r.borrow().right);
                let left = r.borrow_mut().left.take();

                if let Some(node) = successor {
                    node.borrow_mut().left = left;
                }

                return r.borrow_mut().right.take();
            }

            return Some(r);
        }

        None
    }

    fn find_node(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            if r.borrow().left.is_some() {
                return Self::find_node(&r.borrow().left);
            }
        }

        root.clone()
    }
}

#[test]
fn it_works() {
    use assist::tree;
    let root = tree!(5, tree!(3, tree!(2), tree!(4)), tree!(6, None, tree!(7)));
    let key = 3;
    let res = tree!(5, tree!(4, tree!(2), None), tree!(6, None, tree!(7)));
    assert_eq!(Solution::delete_node(root, key), res);
}
