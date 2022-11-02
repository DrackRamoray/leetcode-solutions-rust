use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        if let Some(ref rf) = root {
            let mut val = rf.borrow().val;
            let mut closet = val;
            let mut r = root;

            while let Some(ref node) = r {
                val = node.borrow().val;
                closet = if (val as f64 - target).abs() < (closet as f64 - target).abs() { val } else { closet };
                r = if target < val as f64 { node.borrow_mut().left.take() } else { node.borrow_mut().right.take() };
            }

            closet
        } else {
            -1
        }
    }
}

#[test]
fn it_works() {
    use assist::tree;
    let root = tree!(4, tree!(2, tree!(1), tree!(3)), tree!(5));
    assert_eq!(Solution::closest_value(root, 3.714286), 4);
}
