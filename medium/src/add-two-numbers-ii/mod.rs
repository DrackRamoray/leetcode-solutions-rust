use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut s1 = vec![];
        let mut s2 = vec![];
        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();

        while let Some(n1) = p1 {
            s1.push(n1);
            p1 = n1.next.as_ref();
        }

        while let Some(n2) = p2 {
            s2.push(n2);
            p2 = n2.next.as_ref();
        }

        let mut ptr = None;
        let mut c = 0;

        while !s1.is_empty() || !s2.is_empty() {
            let v = match (s1.pop(), s2.pop()) {
                (Some(n1), Some(n2)) => n1.val + n2.val + c,
                (Some(n1), None) => n1.val + c,
                (None, Some(n2)) => n2.val + c,
                _ => 0,
            };
            let mut n = Box::new(ListNode::new(v % 10));
            n.next = ptr;
            ptr = Some(n);
            c = v / 10;
        }

        if c != 0 {
            let mut n = Box::new(ListNode::new(c));
            n.next = ptr;
            ptr = Some(n);
        }

        ptr
    }
}

#[test]
fn it_works() {
    use assist::list;

    assert_eq!(Solution::add_two_numbers(list!(7,2,4,3), list!(5,6,4)), list!(7,8,0,7));
    assert_eq!(Solution::add_two_numbers(list!(2,4,3), list!(5,6,4)), list!(8,0,7));
    assert_eq!(Solution::add_two_numbers(list!(0), list!(0)), list!(0));
}
