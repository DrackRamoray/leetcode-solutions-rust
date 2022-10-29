use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut reverse = false;
        let mut stack = vec![root];
        let mut ans = vec![];

        while stack.len() > 0 {
            let mut tmp = vec![];
            let mut vals = vec![];

            while let Some(node) = stack.pop() {
                if let Some(n) = node {
                    vals.push(n.borrow().val);

                    if reverse {
                        tmp.push(n.borrow_mut().right.take());
                        tmp.push(n.borrow_mut().left.take());
                    } else {
                        tmp.push(n.borrow_mut().left.take());
                        tmp.push(n.borrow_mut().right.take());
                    }
                }
            }

            ans.push(vals);
            reverse = !reverse;
            stack = tmp;
        }

        ans.pop();
        ans
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
            3,
            tree!(9),
            tree!(20, tree!(15), tree!(7))
        );
        assert_eq!(Solution::zigzag_level_order(root), vec![vec![3],vec![20,9],vec![15,7]]);
        let root = tree!(1);
        assert_eq!(Solution::zigzag_level_order(root), vec![vec![1]]);
        let root = None;
        assert_eq!(Solution::zigzag_level_order(root), vec![] as Vec<Vec<i32>>);
    }
}
