### [94. 二叉树的中序遍历](https://leetcode.cn/problems/binary-tree-inorder-traversal/)

给定一个二叉树的根节点 root ，返回 它的 中序 遍历 。



##### 示例 1：
![img.png](img.png)
```
输入：root = [1,null,2,3]
输出：[1,3,2]
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
- 树中节点数目在范围 [0, 100] 内
- -100 <= Node.val <= 100


##### 进阶: 
- 递归算法很简单，你可以通过迭代算法完成吗？

##### 题解:
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        let mut node = root;

        while node.is_some() || stack.len() > 0 {
            while let Some(n) = node {
                node = n.borrow_mut().left.take();
                stack.push(n);
            }

            if let Some(n) = stack.pop() {
                ans.push(n.borrow().val);
                node = n.borrow_mut().right.take();
            }
        }

        ans
    }
}
```
