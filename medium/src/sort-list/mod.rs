use assist::ListNode;

struct Solution;

impl Solution {
    fn divide_lists(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut left = Some(Box::new(ListNode::new(0)));
        let mut right = Some(Box::new(ListNode::new(0)));
        let mut flag = true;
        let mut left_ptr = &mut left;
        let mut right_ptr = &mut right;
        while let Some(mut next_node) = head {
            head = next_node.next.take();
            if flag {
                left_ptr.as_mut().unwrap().next = Some(next_node);
                left_ptr = &mut left_ptr.as_mut().unwrap().next;
            } else {
                right_ptr.as_mut().unwrap().next = Some(next_node);
                right_ptr = &mut right_ptr.as_mut().unwrap().next;
            }
            flag = !flag;
        }
        (left.unwrap().next, right.unwrap().next)
    }

    fn merge_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(node1), None) => {
                Some(node1)
            },
            (None, Some(node2)) => {
                Some(node2)
            },
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    node1.next = Self::merge_lists(node1.next, Some(node2));
                    Some(node1)
                } else {
                    node2.next = Self::merge_lists(Some(node1), node2.next);
                    Some(node2)
                }
            },
            _ => None
        }
    }

    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let result = Self::divide_lists(head);
        match result {
            (Some(left_node), Some(right_node)) => {
                let left = Self::sort_list(Some(left_node));
                let right = Self::sort_list(Some(right_node));
                let head = Self::merge_lists(left, right);
                head
            },
            (Some(left_node), None) => {
                Some(left_node)
            },
            (None, Some(right_node)) => {
                Some(right_node)
            },
            _ => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use assist::ListNode;
    use super::Solution;

    #[test]
    fn it_works() {
        let head = ListNode::from_vec(vec![4,2,1,3]);
        let ans = ListNode::from_vec(vec![1,2,3,4]);
        assert_eq!(Solution::sort_list(head), ans);

        let head = ListNode::from_vec(vec![-1,5,3,4,0]);
        let ans = ListNode::from_vec(vec![-1,0,3,4,5]);
        assert_eq!(Solution::sort_list(head), ans);

        let head = None;
        let ans = None;
        assert_eq!(Solution::sort_list(head), ans);
    }
}
