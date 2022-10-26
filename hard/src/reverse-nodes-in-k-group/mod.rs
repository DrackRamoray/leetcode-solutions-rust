use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut p = &mut head;

        for _ in 0..k {
            if let Some(n) = p {
                p = &mut n.next;
            } else {
                return head;
            }
        }

        let mut ans = Self::reverse_k_group(p.take(), k);

        while let Some(h) = head.take() {
            ans = Some(Box::new(ListNode {
                val: h.val,
                next: ans,
            }));
            head = h.next;
        }

        ans
    }
}

#[cfg(test)]
mod tests{
    use assist::ListNode;
    use super::Solution;

    #[test]
    fn it_works() {
        let h1 = Some(Box::new(ListNode{ val: 1, next: Some(Box::new(ListNode{ val: 2, next: Some(Box::new(ListNode{ val: 3, next: Some(Box::new(ListNode{ val: 4, next: Some(Box::new(ListNode::new(5))) })) })) })) }));
        let ans1 = Some(Box::new(ListNode{ val: 2, next: Some(Box::new(ListNode{ val: 1, next: Some(Box::new(ListNode{ val: 4, next: Some(Box::new(ListNode{ val: 3, next: Some(Box::new(ListNode::new(5))) })) })) })) }));
        let ans2 = Some(Box::new(ListNode{ val: 3, next: Some(Box::new(ListNode{ val: 2, next: Some(Box::new(ListNode{ val: 1, next: Some(Box::new(ListNode{ val: 4, next: Some(Box::new(ListNode::new(5))) })) })) })) }));
        assert_eq!(Solution::reverse_k_group(h1.clone(), 2), ans1);
        assert_eq!(Solution::reverse_k_group(h1, 3), ans2);
    }
}
