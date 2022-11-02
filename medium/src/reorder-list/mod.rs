use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut vec = std::collections::VecDeque::new();
        let mut p = head.take();

        while let Some(mut n) = p {
            p = n.next.take();
            vec.push_back(n);
        }

        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut p = dummy.as_mut();

        while vec.len() > 0 {
            p.as_mut().unwrap().next = vec.pop_front();
            p = p.unwrap().next.as_mut();
            p.as_mut().unwrap().next = vec.pop_back();
            p = p.unwrap().next.as_mut();
        }

        *head = dummy.unwrap().next;
    }
}

#[cfg(test)]
mod tests {
    use assist::ListNode;
    use super::Solution;

    #[test]
    fn it_works() {
        let mut head = ListNode::from_vec(vec![1,2,3,4]);
        let ans = ListNode::from_vec(vec![1,4,2,3]);
        Solution::reorder_list(&mut head);
        assert_eq!(head, ans);

        let mut head = ListNode::from_vec(vec![1,2,3,4,5]);
        let ans = ListNode::from_vec(vec![1,5,2,4,3]);
        Solution::reorder_list(&mut head);
        assert_eq!(head, ans);
    }
}
