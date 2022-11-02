use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut small = Box::new(ListNode::new(-201));
        let mut large = Box::new(ListNode::new(201));
        let mut ps = small.as_mut();
        let mut pl = large.as_mut();

        while let Some(mut h) = head {
            head = h.next.take();

            if h.val < x {
                ps.next = Some(h);
                ps = ps.next.as_mut().unwrap();
            } else {
                pl.next = Some(h);
                pl = pl.next.as_mut().unwrap();
            }
        }

        ps.next = large.next;

        small.next
    }
}

#[cfg(test)]
mod tests {
    use assist::ListNode;
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::partition(ListNode::from_vec(vec![1,4,3,2,5,2]), 3), ListNode::from_vec(vec![1,2,2,4,3,5]));
        assert_eq!(Solution::partition(ListNode::from_vec(vec![2,1]), 2), ListNode::from_vec(vec![1,2]));
    }
}
