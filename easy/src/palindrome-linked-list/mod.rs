use assist::ListNode;

struct List {
    head: Option<Box<ListNode>>,
}

impl List {
    fn new(head: Option<Box<ListNode>>) -> Self {
        List { head }
    }

    fn pop(&mut self) -> Option<i32> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.val)
        } else {
            None
        }
    }

    fn into_iter(self) -> IntoIter {
        IntoIter { list: self }
    }
}

struct IntoIter {
    list: List,
}

impl Iterator for IntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

struct Solution;

impl Solution {
    fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let list = List::new(head);
        let vec: Vec<i32> = list.into_iter().collect();
        for (i, &v) in vec.iter().rev().enumerate() {
            if v != vec[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use assist::{ListNode, list};
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_palindrome(list!(1,2,2,1)), true);
        assert_eq!(Solution::is_palindrome(list!(1,2)), false);
    }
}
