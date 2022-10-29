use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut head: Box<ListNode> = Box::new(ListNode::new(0));
        let mut p = &mut head;
        let mut n1 = l1;
        let mut n2 = l2;

        while n1.is_some() || n2.is_some() {
            let mut sum: i32 = 0;

            if let Some(v1) = n1 {
                sum = sum + v1.val;
                n1 = v1.next;
            }

            if let Some(v2) = n2 {
                sum = sum + v2.val;
                n2 = v2.next;
            }

            sum = sum + carry;
            carry = sum / 10;

            p.next = Some(Box::new(ListNode::new(sum % 10)));
            p = p.next.as_mut().unwrap();
        }

        if carry > 0 {
            p.next = Some(Box::new(ListNode::new(carry)));
        }

        head.next
    }
}

#[cfg(test)]
mod tests {
    use assist::ListNode;
    use super::Solution;

    #[test]
    fn it_works() {
        let l1 = ListNode::from_vec(vec![2,4,3]);
        let l2 = ListNode::from_vec(vec![5,6,4]);
        let ans = ListNode::from_vec(vec![7,0,8]);
        assert_eq!(Solution::add_two_numbers(l1, l2), ans);
        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let ans = ListNode::from_vec(vec![0]);
        assert_eq!(Solution::add_two_numbers(l1, l2), ans);
        let l1 = ListNode::from_vec(vec![9,9,9,9,9,9,9]);
        let l2 = ListNode::from_vec(vec![9,9,9,9]);
        let ans = ListNode::from_vec(vec![8,9,9,9,0,0,0,1]);
        assert_eq!(Solution::add_two_numbers(l1, l2), ans);
    }
}
