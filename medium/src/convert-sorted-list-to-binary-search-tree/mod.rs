use assist::{ListNode, TreeNode};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut n = 0;
        let mut p = head.as_ref();
        println!("00000000 {}", head.is_none());
        while let Some(h) = p {

            n += 1;
            p = h.next.as_ref();
        }

        Self::construct(&mut head, n)
    }

    fn construct(head: &mut Option<Box<ListNode>>, n: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if n == 0 {
            return None;
        }

        let left = Self::construct(head, n / 2);

        if let Some(h) = head {
            let mut node = TreeNode::new(h.val);
            *head = h.next.take();
            node.left = left;
            node.right = Self::construct(head, n - n / 2 - 1);
            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use assist::{ListNode, TreeNode, tree};
    use std::rc::Rc;
    use std::cell::RefCell;
    use super::Solution;

    #[test]
    fn it_works() {
        let head = ListNode::from_vec(vec![-10,-3,0,5,9]);
        let root = tree!(
            0,
            tree!(-3, tree!(-10), None),
            tree!(9, tree!(5), None)
        );

        assert_eq!(Solution::sorted_list_to_bst(head), root);

        let head = None;
        let root = None;
        assert_eq!(Solution::sorted_list_to_bst(head), root);
    }
}
