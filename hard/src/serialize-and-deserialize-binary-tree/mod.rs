use std::rc::Rc;
use std::cell::RefCell;
use std::str::SplitWhitespace;
use assist::TreeNode;

pub struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.pre_order(&root)
    }

    fn pre_order(&self, root: &Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(node) = root {
            format!(
                "{} {} {}",
                node.borrow().val.to_string(),
                self.pre_order(&node.borrow().left),
                self.pre_order(&node.borrow().right),
            )
        } else {
            ",".to_string()
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() == 0 {
            return None;
        }

        let mut vec = data.split_whitespace();

        self.constructor(&mut vec)
    }

    fn constructor(&self, iter: &mut SplitWhitespace) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(val) = iter.next() {
            if val != "," {
                let node = Rc::new(RefCell::new(TreeNode::new(val.parse::<i32>().unwrap())));
                node.borrow_mut().left = self.constructor(iter);
                node.borrow_mut().right = self.constructor(iter);
                Some(node)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[test]
fn it_works() {
    use assist::tree;

    let codec = Codec::new();
    let root = tree!(1, tree!(2), tree!(3, tree!(4), tree!(5)));
    let res = tree!(1, tree!(2), tree!(3, tree!(4), tree!(5)));
    assert_eq!(codec.deserialize(codec.serialize(root)), res);
}
