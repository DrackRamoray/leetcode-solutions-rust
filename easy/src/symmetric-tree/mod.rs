use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(r) = root {
            let mut queue = VecDeque::new();
            queue.push_back(r.borrow_mut().left.take());
            queue.push_back(r.borrow_mut().right.take());

            while queue.len() > 0 {
                match (queue.pop_front(), queue.pop_front()) {
                    (Some(Some(n1)), Some(Some(n2))) => {
                        if n1.borrow().val != n2.borrow().val {
                            return false;
                        }

                        queue.push_back(n1.borrow_mut().left.take());
                        queue.push_back(n2.borrow_mut().right.take());

                        queue.push_back(n1.borrow_mut().right.take());
                        queue.push_back(n2.borrow_mut().left.take());
                    },
                    (Some(None), Some(None)) => {},
                    (None, None) => {},
                    _ => {
                        return false;
                    }
                }
            }

            true
        } else {
            true
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
            1,
            tree!(2, tree!(3), tree!(4)),
            tree!(2, tree!(4), tree!(3))
        );
        assert_eq!(Solution::is_symmetric(root), true);

        let root = tree!(
            1,
            tree!(2, None, tree!(3)),
            tree!(2, None, tree!(3))
        );
        assert_eq!(Solution::is_symmetric(root), false);
    }
}
