### [156. 上下翻转二叉树](https://leetcode.cn/problems/binary-tree-upside-down/)

##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn upside_down_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::pre_order(root, None, None)
    }   

    fn pre_order(root: Option<Rc<RefCell<TreeNode>>>, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            let left_sub_tree = r.borrow_mut().left.take();
            let right_sub_tree = r.borrow_mut().right.take();
            r.borrow_mut().left = left;
            r.borrow_mut().right = right;

            Self::pre_order(left_sub_tree, right_sub_tree, Some(r))
        } else {
            right
        }
    }
}
```
