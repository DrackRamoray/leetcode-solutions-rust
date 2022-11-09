use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn plus_one(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let carry = Self::plus(head.as_mut());

        if carry == 0 {
            head
        } else {
            let mut new_head = Box::new(ListNode::new(carry));
            new_head.as_mut().next = head;
            Some(new_head)
        }
    }

    fn plus(head: Option<&mut Box<ListNode>>) -> i32 {
        if let Some(h)= head {
            let val = h.val + Self::plus(h.next.as_mut());
            h.as_mut().val = val % 10;
            val / 10
        } else {
            1
        }
    }
}

#[test]
fn it_works() {
    use assist::list;

    assert_eq!(Solution::plus_one(list!(1,2,3)), list!(1,2,4));
    assert_eq!(Solution::plus_one(list!(0)), list!(1));
}
