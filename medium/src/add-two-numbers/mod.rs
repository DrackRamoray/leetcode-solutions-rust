use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut head: Box<ListNode> = Box::new(ListNode::new(0));
        let mut p = &mut head;
        let mut n1 = l1;
        let mut n2 = l2;

        while n1.is_some() || n2.is_some() {
            let mut sum: i32 = 0;

            if let Some(v1) = n1 {
                sum = sum + v1.val;
                n1 = v1.next;
            }

            if let Some(v2) = n2 {
                sum = sum + v2.val;
                n2 = v2.next;
            }

            sum = sum + carry;
            carry = sum / 10;

            p.next = Some(Box::new(ListNode::new(sum % 10)));
            p = p.next.as_mut().unwrap();
        }

        if carry > 0 {
            p.next = Some(Box::new(ListNode::new(carry)));
        }

        head.next
    }
}

#[cfg(test)]
mod tests {
    use assist::ListNode;
    use super::Solution;

    #[test]
    fn it_works() {
        let l1 = Some(Box::new(ListNode{ val: 2, next: Some(Box::new(ListNode{ val: 4, next: Some(Box::new(ListNode::new(3))) })) }));
        let l2 = Some(Box::new(ListNode{ val: 5, next: Some(Box::new(ListNode{ val: 6, next: Some(Box::new(ListNode::new(4))) })) }));
        let ans = Some(Box::new(ListNode{ val: 7, next: Some(Box::new(ListNode{ val: 0, next: Some(Box::new(ListNode::new(8))) })) }));
        assert_eq!(Solution::add_two_numbers(l1, l2), ans);
        let l1 = Some(Box::new(ListNode{ val: 0, next: None }));
        let l2 = Some(Box::new(ListNode{ val: 0, next: None }));
        let ans = Some(Box::new(ListNode{ val: 0, next: None }));
        assert_eq!(Solution::add_two_numbers(l1, l2), ans);
        let l1 = Some(Box::new(ListNode{ val: 9, next: Some(Box::new(ListNode{ val: 9, next: Some(Box::new( ListNode{ val:9, next: Some(Box::new(ListNode{ val: 9, next: Some(Box::new(ListNode{ val: 9, next: Some(Box::new( ListNode{ val:9, next: Some(Box::new(ListNode::new(9))) })) })) })) })) })) }));
        let l2 = Some(Box::new(ListNode{ val: 9, next: Some(Box::new(ListNode{ val: 9, next: Some(Box::new( ListNode{ val:9, next: Some(Box::new(ListNode::new(9))) })) })) }));
        let ans = Some(Box::new(ListNode{ val: 8, next: Some(Box::new(ListNode{ val: 9, next: Some(Box::new( ListNode{ val:9, next: Some(Box::new(ListNode{ val: 9, next: Some(Box::new(ListNode{ val: 0, next: Some(Box::new( ListNode{ val:0, next: Some(Box::new(ListNode{ val: 0, next: Some(Box::new(ListNode::new(1))) })) })) })) })) })) })) }));
        assert_eq!(Solution::add_two_numbers(l1, l2), ans);
    }
}
