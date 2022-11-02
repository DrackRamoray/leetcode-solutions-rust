use assist::ListNode;

struct Solution;

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
        let head = ListNode::from_vec(vec![1,2,3,4]);
        let ans = ListNode::from_vec(vec![2,1,4,3]);
        assert_eq!(Solution::swap_pairs(head), ans);
        assert_eq!(Solution::swap_pairs(None), None);
        assert_eq!(Solution::swap_pairs(ListNode::from_vec(vec![1])), ListNode::from_vec(vec![1]));
    }
}
