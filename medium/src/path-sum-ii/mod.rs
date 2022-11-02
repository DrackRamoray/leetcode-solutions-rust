use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        Self::dfs(&root, target_sum, &mut ans, &mut Vec::new());
        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mut target_sum: i32, ans: &mut Vec<Vec<i32>>, selected: &mut Vec<i32>) {
        if let Some(r) = root {
            target_sum -= r.borrow().val;
            selected.push(r.borrow().val);

            if r.borrow().left.is_none() && r.borrow().right.is_none() && target_sum == 0 {
                ans.push(selected.to_vec());
            }

            Self::dfs(&r.borrow().left, target_sum, ans, selected);
            Self::dfs(&r.borrow().right, target_sum, ans, selected);
            selected.pop();
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
            5,
            tree!(4, tree!(11, tree!(7), tree!(2)), None),
            tree!(8, tree!(13), tree!(4, tree!(5), tree!(1)))
        );
        assert_eq!(Solution::path_sum(root, 22), vec![vec![5,4,11,2],vec![5,8,4,5]]);

        let root = tree!(
            1,
            tree!(2),
            tree!(3)
        );
        assert_eq!(Solution::path_sum(root, 5), vec![] as Vec<Vec<i32>>);

        let root = tree!(
            1,
            tree!(2),
            None
        );
        assert_eq!(Solution::path_sum(root, 0), vec![] as Vec<Vec<i32>>);
    }
}
