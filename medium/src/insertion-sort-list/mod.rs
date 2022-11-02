use assist::ListNode;

struct Solution;

impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;

        while let Some(mut h) = head {
            head = h.next.take();
            prev = Self::insert(prev, Some(h))
        }

        prev
    }

    fn insert(list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let val = list2.as_ref().unwrap().val;

        if let Some(mut n) = list1 {
            if n.val > val {
                list2.as_mut().unwrap().next = Some(n);
                list2
            } else {
                n.next = Self::insert(n.next.take(), list2);
                Some(n)
            }
        } else {
            list2
        }
    }
}

#[cfg(test)]
mod tests {
    use assist::ListNode;
    use super::Solution;

    #[test]
    fn it_works() {
        let head = ListNode::from_vec(vec![4, 2, 1, 3]);
        let ans = ListNode::from_vec(vec![1, 2, 3, 4]);
        assert_eq!(Solution::insertion_sort_list(head), ans);

        let head = ListNode::from_vec(vec![-1, 5, 3, 4, 0]);
        let ans = ListNode::from_vec(vec![-1, 0, 3, 4, 5]);
        assert_eq!(Solution::insertion_sort_list(head), ans);
    }
}
