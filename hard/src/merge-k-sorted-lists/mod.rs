use assist::ListNode;

pub struct Solution;

impl Solution {
    fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut lists = lists;

        while lists.len() > 1 {
            if let (Some(a), Some(b)) = (lists.pop(), lists.pop()) {
                let merged_list = Self::merge(a, b);
                lists.push(merged_list);
            }
        }

        lists.pop().unwrap_or(None)
    }

    fn merge(a: Option<Box<ListNode>>, b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (a, b) {
            (Some(mut x), Some(mut y)) => {
                if x.val < y.val {
                    x.next = Self::merge(x.next.take(), Some(y));
                    Some(x)
                } else {
                    y.next = Self::merge(Some(x), y.next.take());
                    Some(y)
                }
            },
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use assist::ListNode;
    use super::Solution;

    #[test]
    fn it_works() {
        let lists = vec![
            ListNode::from_vec(vec![1,4,5]),
            ListNode::from_vec(vec![1,3,4]),
            ListNode::from_vec(vec![2,6]),
        ];
        let ans = ListNode::from_vec(vec![1,1,2,3,4,4,5,6]);
        assert_eq!(Solution::merge_k_lists(lists), ans);
        assert_eq!(Solution::merge_k_lists(vec![]), None);
        assert_eq!(Solution::merge_k_lists(vec![None]), None);
    }
}
