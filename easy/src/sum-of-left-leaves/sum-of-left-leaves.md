### [404. 左叶子之和](https://leetcode.cn/problems/sum-of-left-leaves/)
给定二叉树的根节点 root ，返回所有左叶子之和。



##### 示例 1：
![img.png](img.png)
```
输入: root = [3,9,20,null,null,15,7]
输出: 24
解释: 在这个二叉树中，有两个左叶子，分别是 9 和 15，所以返回 24
```

##### 示例 2:
```
输入: root = [1]
输出: 0
```

##### 提示:
- 节点数在 [1, 1000] 范围内
- -1000 <= Node.val <= 1000

##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum(&root)
    }

    fn sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut total = 0;
        if let Some(r) = root {
            if let Some(left) = &r.borrow().left {
                if left.borrow().left.is_none() && left.borrow().right.is_none() {
                    total += left.borrow().val;
                }
            }

            total += Self::sum(&r.borrow().left);
            total += Self::sum(&r.borrow().right);
        }

        total
    }
}
```
