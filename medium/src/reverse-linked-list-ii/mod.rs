use assist::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut p1 = &mut head;

        for _ in 1..left {
            if let Some(n) = p1 {
                p1 = &mut n.next;
            } else {
                break;
            }
        }

        let mut p2 = p1.take();
        let mut p3 = &mut p2;

        for _ in left..=right {
            if let Some(n) = p3 {
                p3 = &mut n.next;
            } else {
                break;
            }
        }

        let mut p4 = p3.take();

        while let Some(mut p) = p2 {
            p2 = p.next.take();
            p.next = p4;
            p4 = Some(p);
        }

        *p1 = p4;

        head
    }
}

#[cfg(test)]
mod tests {
    use assist::ListNode;
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_between(ListNode::from_vec(vec![1,2,3,4,5]), 2, 4), ListNode::from_vec(vec![1,4,3,2,5]));
        assert_eq!(Solution::reverse_between(ListNode::from_vec(vec![5]), 1, 1), ListNode::from_vec(vec![5]));
    }
}
