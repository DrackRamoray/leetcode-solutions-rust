use assist::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct BSTIterator {
    index: usize,
    data: Vec<i32>,
}

impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut data = vec![];
        let mut stack = vec![];
        let mut node = root;

        while node.is_some() || stack.len() > 0 {
            while let Some(n) = node {
                node = n.borrow_mut().left.take();
                stack.push(n);
            }

            if let Some(n) = stack.pop() {
                data.push(n.borrow().val);
                node = n.borrow_mut().right.take();
            }
        }

        Self {
            index: 0,
            data,
        }
    }

    fn next(&mut self) -> i32 {
        if self.index == self.data.len() {
            return -1;
        }

        let v = self.data[self.index];
        self.index += 1;
        v
    }

    fn has_next(&self) -> bool {
        self.index < self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use assist::{TreeNode, tree};
    use std::rc::Rc;
    use std::cell::RefCell;
    use super::BSTIterator;

    #[test]
    fn it_works() {
        let root = tree!(7, tree!(3), tree!(15, tree!(9), tree!(20)));
        let mut it = BSTIterator::new(root);
        assert_eq!(it.next(), 3);
        assert_eq!(it.next(), 7);
        assert_eq!(it.has_next(), true);
        assert_eq!(it.next(), 9);
        assert_eq!(it.has_next(), true);
        assert_eq!(it.next(), 15);
        assert_eq!(it.has_next(), true);
        assert_eq!(it.next(), 20);
        assert_eq!(it.has_next(), false);
    }
}
