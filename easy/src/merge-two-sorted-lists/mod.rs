use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(mut l1), Some(mut l2)) => {
                if l1.val <= l2.val {
                    l1.next = Self::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    l2.next = Self::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            },
            (Some(l1), None) => {
                Some(l1)
            },
            (None, Some(l2)) => {
                Some(l2)
            },
            (None, None) => {
                None
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use assist::ListNode;

    #[test]
    fn it_works() {
        let l1 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode::new(4)))}))}));
        let l2 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode::new(4))) }))}));
        let l3 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode::new(4))),}))}))}))}))}));
        assert_eq!(Solution::merge_two_lists(l1, l2), l3);
        assert_eq!(Solution::merge_two_lists(None, None), None);
        assert_eq!(Solution::merge_two_lists(None, Some(Box::new(ListNode::new(1)))), Some(Box::new(ListNode::new(1))));
    }
}
