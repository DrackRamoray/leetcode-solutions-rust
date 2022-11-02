use assist::ListNode;

struct Solution;

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }

        let mut len = 0;
        let mut cur = head.as_ref();

        while let Some(n) = cur {
            len += 1;
            cur = n.next.as_ref();
        }

        if k == len {
            return head;
        }

        let mut k = len - k % len;
        let mut cur = head.as_mut();
        let mut tail = None;

        while let Some(n) = cur {
            if k == 1 {
                tail = n.next.take();
                break;
            }
            cur = n.next.as_mut();
            k -= 1;
        }

        if tail.is_none() {
            return head;
        }

        let mut cur = tail.as_mut();

        while let Some(n) = cur {
            if n.next.is_none() {
                n.next = head;
                break;
            }
            cur = n.next.as_mut();
        }

        tail
    }
}

#[cfg(test)]
mod tests {
    use assist::ListNode;
    use super::Solution;

    #[test]
    fn it_works() {
        let head = ListNode::from_vec(vec![1,2,3,4,5]);
        let ans = ListNode::from_vec(vec![4,5,1,2,3]);
        assert_eq!(Solution::rotate_right(head, 2), ans);
        let head = ListNode::from_vec(vec![0,1,2]);
        let ans = ListNode::from_vec(vec![2,0,1]);
        assert_eq!(Solution::rotate_right(head, 4), ans);
    }
}
