use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut stack = vec![];

        if let Some(r) = root {
            let mut mp = BTreeMap::new();
            let mut tmp = vec![];
            stack.push((r, 0));

            while stack.len() > 0 {
                for i in 0..stack.len() {
                    mp.entry(stack[i].1).or_insert(vec![]).push(stack[i].0.borrow().val);

                    if let Some(left) = stack[i].0.borrow_mut().left.take() {
                        tmp.push((left, stack[i].1 - 1));
                    }

                    if let Some(right) = stack[i].0.borrow_mut().right.take() {
                        tmp.push((right, stack[i].1 + 1));
                    }
                }

                stack = tmp;
                tmp = vec![];
            }

            mp.into_values().collect()
        } else {
            vec![]
        }
    }
}

#[test]
fn it_works() {
    use assist::tree;

    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let ans = vec![vec![9],vec![3,15],vec![20],vec![7]];
    assert_eq!(Solution::vertical_order(root), ans);

    let root = tree!(3, tree!(9, tree!(4), tree!(0)), tree!(8, tree!(1), tree!(7)));
    let ans = vec![vec![4],vec![9],vec![3,0,1],vec![8],vec![7]];
    assert_eq!(Solution::vertical_order(root), ans);

    let root = tree!(3, tree!(9, tree!(4), tree!(0, tree!(5), None)), tree!(8, tree!(1, None, tree!(2)), tree!(7)));
    let ans = vec![vec![4],vec![9,5],vec![3,0,1],vec![8,2],vec![7]];
    assert_eq!(Solution::vertical_order(root), ans);
}
