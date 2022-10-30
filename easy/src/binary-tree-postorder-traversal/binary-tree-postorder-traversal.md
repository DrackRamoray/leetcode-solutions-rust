### [145. 二叉树的后序遍历](https://leetcode.cn/problems/binary-tree-postorder-traversal/)

给你一棵二叉树的根节点 root ，返回其节点值的 后序遍历 。



##### 示例 1：
![img.png](img.png)
```
输入：root = [1,null,2,3]
输出：[3,2,1]
```

##### 示例 2：
```
输入：root = []
输出：[]
```

##### 示例 3：
```
输入：root = [1]
输出：[1]
```

##### 提示：
- 树中节点的数目在范围 [0, 100] 内
- -100 <= Node.val <= 100

##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        let mut node = root;
        let mut prev = None;

        while node.is_some() || stack.len() > 0 {
            while let Some(n) = node {
                node = n.borrow_mut().left.take();
                stack.push(n);
            }

            if let Some(n) = stack.pop() {
                if n.borrow().right.is_none() || n.borrow().right == prev {
                    ans.push(n.borrow().val);
                    prev = Some(n);
                    node = None;
                } else {
                    node = n.borrow_mut().right.take();
                    stack.push(n);
                }
            }
        }

        ans
    }
 }
```
