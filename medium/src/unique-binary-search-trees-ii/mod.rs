use assist::{TreeNode, tree};
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return Vec::new()
        }

        Self::generate(1, n)
    }

    fn generate(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            return vec![None];
        }

        let mut ans = vec![];

        for i in start..=end {
            let left = Self::generate(start, i - 1);
            let right = Self::generate(i + 1, end);

            for left_node in left.iter() {
                for right_node in right.iter() {
                    ans.push(tree!(i, left_node.clone(), right_node.clone()));
                }
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
        let ans = vec![
            tree!(1, None, tree!(2, None, tree!(3))),
            tree!(1, None, tree!(3, tree!(2), None)),
            tree!(2, tree!(1), tree!(3)),
            tree!(3, tree!(1, None, tree!(2)), None),
            tree!(3, tree!(2, tree!(1), None), None),
        ];
        assert_eq!(Solution::generate_trees(3), ans);
    }
}
