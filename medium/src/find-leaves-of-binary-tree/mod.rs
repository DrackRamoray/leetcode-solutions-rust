use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(&root, &mut ans);

        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<Vec<i32>>) -> i32 {
        if let Some(r) = root {
            let left = Self::dfs(&r.borrow().left, ans);
            let right = Self::dfs(&r.borrow().right, ans);
            let d = left.max(right) + 1;

            if ans.len() <= d as usize {
                ans.push(vec![]);
            }

            ans[d as usize].push(r.borrow().val);

            d
        } else {
            -1
        }
    }
}

#[test]
fn it_works() {
    use assist::tree;

    let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3));
    assert_eq!(Solution::find_leaves(root), vec![vec![4,5,3],vec![2], vec![1]]);
}
