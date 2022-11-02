use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(mut root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (p, q) {
            (Some(p), Some(q)) => {
                while let Some(r) = root {
                    if r.borrow().val > p.borrow().val && r.borrow().val > q.borrow().val {
                        root = r.borrow_mut().left.take();
                    } else if r.borrow().val < p.borrow().val && r.borrow().val < q.borrow().val {
                        root = r.borrow_mut().right.take();
                    } else {
                        return Some(r);
                    }
                }

                None
            },
            _ => None,
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
        let root = tree!(6, tree!(2, tree!(0), tree!(4, tree!(3), tree!(5))), tree!(8, tree!(7), tree!(8)));
        let p = tree!(2);
        let q = tree!(8);
        assert_eq!(Solution::lowest_common_ancestor(root.clone(), p, q), root.clone());

        let root = root.clone();
        let p = tree!(2);
        let q = tree!(4);
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), tree!(2, tree!(0), tree!(4, tree!(3), tree!(5))));
    }
}
