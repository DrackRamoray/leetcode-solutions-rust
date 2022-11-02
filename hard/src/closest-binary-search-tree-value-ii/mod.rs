use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn closest_k_values(root: Option<Rc<RefCell<TreeNode>>>, target: f64, k: i32) -> Vec<i32> {
        let mut queue = VecDeque::new();

        Self::inorder(&mut queue, &root, target, k as usize);

        queue.into_iter().collect::<Vec<_>>()
    }

    fn inorder(queue: &mut VecDeque<i32>, root: &Option<Rc<RefCell<TreeNode>>>, target: f64, k: usize) {
        if let Some(r) = root {
            Self::inorder(queue, &r.borrow().left, target, k);

            if queue.len() == k {
                if (*queue.front().unwrap() as f64 - target).abs() > (r.borrow().val as f64 - target).abs() {
                    queue.pop_front();
                    queue.push_back(r.borrow().val);
                } else {
                    return;
                }
            } else {
                queue.push_back(r.borrow().val);
            }

            Self::inorder(queue, &r.borrow().right, target, k);
        }
    }
}

#[test]
fn it_works() {
    use assist::tree;
    let root = tree!(4,tree!(2,tree!(1),tree!(3)),tree!(5));
    assert_eq!(Solution::closest_k_values(root, 3.714286, 2), vec![3,4]);
    let root = tree!(1);
    assert_eq!(Solution::closest_k_values(root, 0.000000, 1), vec![1]);
}
