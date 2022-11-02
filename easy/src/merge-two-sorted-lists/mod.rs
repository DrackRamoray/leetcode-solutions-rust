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
        let l1 = ListNode::from_vec(vec![1,2,4]);
        let l2 = ListNode::from_vec(vec![1,3,4]);
        let l3 = ListNode::from_vec(vec![1,1,2,3,4,4]);
        assert_eq!(Solution::merge_two_lists(l1, l2), l3);
        assert_eq!(Solution::merge_two_lists(None, None), None);
        assert_eq!(Solution::merge_two_lists(None, ListNode::from_vec(vec![1])), ListNode::from_vec(vec![1]));
    }
}
