use assist::ListNode;

struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head.as_mut();

        while let Some(mut node) = cur.take() {
            match node.next {
                Some(ref mut n) => {
                    if n.val == node.val {
                        node.next = n.next.take();
                        cur = Some(node);
                    } else {
                        cur = node.next.as_mut();
                    }
                },
                None => {
                    cur = node.next.as_mut();
                }
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use assist::ListNode;
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::delete_duplicates(ListNode::from_vec(vec![1,1,2])), ListNode::from_vec(vec![1,2]));
        assert_eq!(Solution::delete_duplicates(ListNode::from_vec(vec![1,1,2,3,3])), ListNode::from_vec(vec![1,2,3]));
    }
}
