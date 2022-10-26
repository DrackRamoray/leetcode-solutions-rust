use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut dummy = Box::new(dummy);

        Self::travel(&mut dummy, n);

        dummy.next
    }

    fn travel(cur: &mut Box<ListNode>, n : i32) -> i32 {
        if let Some(child) = cur.next.as_mut() {
            let num = 1 + Self::travel(child, n);

            if num == n {
                cur.next = child.next.take();
            }

            return num
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use assist::ListNode;

    #[test]
    fn it_works() {
        let head = Some(Box::new(ListNode{ val: 1, next: Some(Box::new(ListNode{ val: 2, next: Some(Box::new(ListNode{ val: 3, next: Some(Box::new(ListNode{ val: 4, next: Some(Box::new(ListNode::new(5))) })) })) })) }));
        let ans = Some(Box::new(ListNode{ val: 1, next: Some(Box::new(ListNode{ val: 2, next: Some(Box::new(ListNode{ val: 3, next: Some(Box::new(ListNode::new(5))) })) })) }));
        assert_eq!(Solution::remove_nth_from_end(head, 2), ans);
        let head = Some(Box::new(ListNode::new(1)));
        let ans = None;
        assert_eq!(Solution::remove_nth_from_end(head, 1), ans);
        let head = Some(Box::new(ListNode{ val: 1, next: Some(Box::new(ListNode::new(2))) }));
        let ans = Some(Box::new(ListNode::new(1)));
        assert_eq!(Solution::remove_nth_from_end(head, 1), ans);
    }
}
