use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let mut stack = vec![];
            let mut tmp = vec![];
            let mut ans = 0;
            stack.push((r, 0));

            while stack.len() > 0 {
                let n = stack.len();
                for i in 0..n {
                    if let Some(left) = stack[i].0.borrow_mut().left.take() {
                        tmp.push((left, stack[i].1 - 1));
                    }

                    if let Some(right) = stack[i].0.borrow_mut().right.take() {
                        tmp.push((right, stack[i].1 + 1));
                    }
                }

                stack = tmp;
                ans += n;
                tmp = vec![];
            }

            ans as i32
        } else {
            0
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
        let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3, tree!(6), None));
        let ans = 6;
        assert_eq!(Solution::count_nodes(root), ans);
    }
}
