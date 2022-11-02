use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ans = Vec::new();
        let mut paths = Vec::new();

        Self::dfs(root, &mut ans, &mut paths);

        ans
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<String>, paths: &mut Vec<String>) {
        if let Some(r) = root {
            let mut node = r.borrow_mut();
            let s = node.val.to_string();

            paths.push(s);

            if node.left.is_none() && node.right.is_none() {
                ans.push(paths.join("->"));
            }

            Self::dfs(node.left.take(), ans, paths);
            Self::dfs(node.right.take(), ans, paths);

            paths.pop();
        }
    }
}

#[test]
fn it_works() {
    use assist::tree;

    let root = tree!(1,tree!(2,None,tree!(5)),tree!(3));
    assert_eq!(Solution::binary_tree_paths(root), vec!["1->2->5".to_string(), "1->3".to_string()]);
    let root = tree!(1);
    assert_eq!(Solution::binary_tree_paths(root), vec!["1".to_string()]);
}
