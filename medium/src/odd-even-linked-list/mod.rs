use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_head = Box::new(ListNode::new(0));
        let mut even_head = Box::new(ListNode::new(0));
        let mut odd = odd_head.as_mut();
        let mut even = even_head.as_mut();

        let mut is_even = false;
        while let Some(mut node) = head {
            head = node.next.take();

            if is_even {
                even.next = Some(node);

                if let Some(ref mut next) = even.next {
                    even = next;
                }
            } else {
                odd.next = Some(node);

                if let Some(ref mut next) = odd.next {
                    odd = next;
                }
            }

            is_even = !is_even;
        }

        odd.next = even_head.next;
        odd_head.next
    }
}

#[test]
fn it_works() {
    use assist::list;

    assert_eq!(Solution::odd_even_list(list!(1,2,3,4,5)), list!(1,3,5,2,4));
    assert_eq!(Solution::odd_even_list(list!(2,1,3,5,6,4,7)), list!(2,3,6,7,1,5,4));
}
