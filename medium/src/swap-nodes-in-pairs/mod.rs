use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut n| {
            match n.next {
                Some(mut m) => {
                    n.next = Self::swap_pairs(m.next);
                    m.next = Some(n);

                    Some(m)
                },
                None => Some(n)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use assist::ListNode;
    use super::Solution;

    #[test]
    fn it_works() {
        let head = Some(Box::new(ListNode{ val: 1, next: Some(Box::new(ListNode{ val: 2, next: Some(Box::new(ListNode{ val: 3, next: Some(Box::new(ListNode::new(4))) })) })) }));
        let ans = Some(Box::new(ListNode{ val: 2, next: Some(Box::new(ListNode{ val: 1, next: Some(Box::new(ListNode{ val: 4, next: Some(Box::new(ListNode::new(3))) })) })) }));
        assert_eq!(Solution::swap_pairs(head), ans);
        assert_eq!(Solution::swap_pairs(None), None);
        assert_eq!(Solution::swap_pairs(Some(Box::new(ListNode::new(1)))), Some(Box::new(ListNode::new(1))));
    }
}
