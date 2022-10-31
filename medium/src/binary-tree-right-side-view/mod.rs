use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut queue = vec![];

        if let Some(r) = root {
            queue.push(r);

            while queue.len() > 0 {
                let mut tmp = vec![];

                for i in 0..queue.len() {
                    if let Some(left) = queue[i].borrow_mut().left.take() {
                        tmp.push(left);
                    }

                    if let Some(right) = queue[i].borrow_mut().right.take() {
                        tmp.push(right);
                    }
                }

                ans.push(queue[queue.len()-1].borrow().val);
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
        let root = tree!(1, tree!(2, None, tree!(5)), tree!(3, None, tree!(4)));
        let ans = vec![1,3,4];
        assert_eq!(Solution::right_side_view(root), ans);

        let root = tree!(1, None, tree!(3));
        let ans = vec![1,3];
        assert_eq!(Solution::right_side_view(root), ans);

        let root = None;
        let ans = vec![] as Vec<i32>;
        assert_eq!(Solution::right_side_view(root), ans);
    }
}
