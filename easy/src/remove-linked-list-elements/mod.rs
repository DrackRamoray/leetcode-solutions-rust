use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut prev = &mut dummy;

        while let Some(n) = prev.next.take() {
            if n.val == val {
                prev.next = n.next;
            } else {
                prev.next = Some(n);
                prev = prev.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use assist::{ListNode, list};
    use super::Solution;

    #[test]
    fn it_works() {
        let head = list!(1,2,3,4,5);
        let ans = list!(1,2,3,4,5);
        assert_eq!(Solution::remove_elements(head, 6), ans);

        let head = None;
        let ans: Option<Box<ListNode>> = None;
        assert_eq!(Solution::remove_elements(head, 1), ans);

        let head = list!(7,7,7,7);
        let ans: Option<Box<ListNode>> = None;
        assert_eq!(Solution::remove_elements(head, 7), ans);
    }
}
