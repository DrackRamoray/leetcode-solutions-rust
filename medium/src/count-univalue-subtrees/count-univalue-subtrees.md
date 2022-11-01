### [250. 统计同值子树](https://leetcode.cn/problems/count-univalue-subtrees/)

##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        Self::is_unival_subtrees(&root, i32::MAX, &mut count);
        count
    }

    fn is_unival_subtrees(root: &Option<Rc<RefCell<TreeNode>>>, val: i32, count: &mut i32) -> bool {
        if let Some(r) = root {
            let b1 = Self::is_unival_subtrees(&r.borrow().left, r.borrow().val, count);
            let b2 = Self::is_unival_subtrees(&r.borrow().right, r.borrow().val, count);

            if !b1 || !b2 {
                return false;
            }

            *count += 1;

            r.borrow().val == val
        } else {
            true
        }
    }
}
```
