use assist::ListNode;

struct Solution;

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
        let h1 = ListNode::from_vec(vec![1,2,3,4,5]);
        let ans1 = ListNode::from_vec(vec![2,1,4,3,5]);
        let ans2 = ListNode::from_vec(vec![3,2,1,4,5]);
        assert_eq!(Solution::reverse_k_group(h1.clone(), 2), ans1);
        assert_eq!(Solution::reverse_k_group(h1, 3), ans2);
    }
}
