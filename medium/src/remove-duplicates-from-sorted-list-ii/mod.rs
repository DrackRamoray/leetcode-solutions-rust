use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut h) => match h.next {
                None => Some(h),
                Some(mut n) => {
                    if h.val != n.val {
                        h.next = Self::delete_duplicates(Some(n));
                        Some(h)
                    } else {
                        while n.val == h.val {
                            match n.next {
                                None => return None,
                                Some(nn) => n = nn,
                            };
                        }

                        Self::delete_duplicates(Some(n))
                    }
                }
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
        assert_eq!(Solution::delete_duplicates(ListNode::from_vec(vec![1,2,3,3,4,4,5])), ListNode::from_vec(vec![1,2,5]));
        assert_eq!(Solution::delete_duplicates(ListNode::from_vec(vec![1,1,1,2,3])), ListNode::from_vec(vec![2,3]));
    }
}
