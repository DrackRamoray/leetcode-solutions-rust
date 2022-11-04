use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut queue = vec![];

        if let Some(r) = root {
            queue.push((r, 1));

            while queue.len() > 0 {
                let mut tmp = vec![];

                for i in 0..queue.len() {
                    let (item, cnt) = &queue[i];
                    let val = item.borrow().val;
                    let cnt = *cnt;
                    ans = ans.max(cnt);

                    if let Some(left) = item.borrow_mut().left.take() {
                        if left.borrow().val == val + 1 {
                            tmp.push((left, cnt + 1));
                        } else {
                            tmp.push((left, 1));
                        }
                    }

                    if let Some(right) = item.borrow_mut().right.take() {
                        if right.borrow().val == val + 1 {
                            tmp.push((right, cnt + 1));
                        } else {
                            tmp.push((right, 1));
                        }
                    }
                }

                queue = tmp;
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    use assist::tree;

    let root = tree!(1, None, tree!(3, tree!(2), tree!(4, None, tree!(5))));
    let res = 3;
    assert_eq!(Solution::longest_consecutive(root), res);
    let root = tree!(2, None, tree!(3, tree!(2, tree!(1), None), None));
    let res = 2;
    assert_eq!(Solution::longest_consecutive(root), res);
}
