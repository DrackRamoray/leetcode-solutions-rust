use assist::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut mp = HashMap::new();
        mp.insert(0, 1);

        Self::count(&root, target_sum as i64, &mut mp, 0)
    }

    fn count(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target_sum: i64,
        prefix_sum: &mut HashMap<i64, i32>,
        cur_sum: i64,
    ) -> i32 {
        if let Some(r) = root {
            let mut ans = 0;
            let cur_sum = cur_sum + r.borrow().val as i64;
            ans += if let Some(&v) = prefix_sum.get(&(cur_sum - target_sum)) {
                v
            } else {
                0
            };

            *prefix_sum.entry(cur_sum).or_insert(0) += 1;

            ans += Self::count(&r.borrow().left, target_sum, prefix_sum, cur_sum);
            ans += Self::count(&r.borrow().right, target_sum, prefix_sum, cur_sum);

            *prefix_sum.entry(cur_sum).or_insert(0) -= 1;

            ans
        } else {
            0
        }
    }
}

#[test]
fn it_works() {
    use assist::tree;

    let root = tree!(
        10,
        tree!(5, tree!(3, tree!(3), tree!(-2)), tree!(2, None, tree!(1))),
        tree!(-3, None, tree!(11))
    );
    assert_eq!(Solution::path_sum(root, 8), 3);
}
