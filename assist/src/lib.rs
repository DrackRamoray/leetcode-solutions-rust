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
