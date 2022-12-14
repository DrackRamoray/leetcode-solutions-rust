use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut queue = vec![];

        if let Some(r) = root {
            queue.push(r);

            while queue.len() > 0 {
                let mut tmp = vec![];
                let mut vals = vec![];

                for i in 0..queue.len() {
                    vals.push(queue[i].borrow().val);

                    if let Some(left) = queue[i].borrow_mut().left.take() {
                        tmp.push(left);
                    }

                    if let Some(right) = queue[i].borrow_mut().right.take() {
                        tmp.push(right);
                    }
                }

                ans.push(vals);
                queue = tmp;
            }
        }

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
        assert_eq!(Solution::level_order(root), vec![vec![3],vec![9,20],vec![15,7]]);

        let root = tree!(1);
        assert_eq!(Solution::level_order(root), vec![vec![1]]);

        let root = None;
        assert_eq!(Solution::level_order(root), vec![] as Vec<Vec<i32>>);
    }
}
