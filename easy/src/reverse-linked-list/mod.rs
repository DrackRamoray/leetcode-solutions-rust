use assist::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut node) = curr.take() {
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use assist::{ListNode, list};
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_list(list!(1,2,3,4,5)), list!(5,4,3,2,1));
        assert_eq!(Solution::reverse_list(list!(1,2)), list!(2,1));
        assert_eq!(Solution::reverse_list(None), None as Option<Box<ListNode>>);
    }
}
