use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }

  pub fn from_vec(mut nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;

    while let Some(num) = nums.pop() {
      if let Some(mut h) = head {
        h.next = Some(Box::new(ListNode::new(num)));
        head = Some(h);
      }
    }

    head
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

#[macro_export]
macro_rules! tree {
    ($v:expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            val: $v,
            left: None,
            right: None,
        })))
    };
    ($v:expr, $l:expr, $r:expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            val: $v,
            left: $l,
            right: $r,
        })))
    };
}
