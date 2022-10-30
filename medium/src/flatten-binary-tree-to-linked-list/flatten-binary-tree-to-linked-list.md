### [114. 二叉树展开为链表](https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/)

给你二叉树的根结点 root ，请你将它展开为一个单链表：

展开后的单链表应该同样使用 TreeNode ，其中 right 子指针指向链表中下一个结点，而左子指针始终为 null 。
展开后的单链表应该与二叉树 先序遍历 顺序相同。


##### 示例 1：
![img.png](img.png)
```
输入：root = [1,2,5,3,4,null,6]
输出：[1,null,2,null,3,null,4,null,5,null,6]
```

##### 示例 2：
```
输入：root = []
输出：[]
```

##### 示例 3：
```
输入：root = [0]
输出：[0]
```

##### 提示：
- 树中结点数在范围 [0, 2000] 内
- -100 <= Node.val <= 100


##### 进阶：
- 你可以使用原地算法（O(1) 额外空间）展开这棵树吗？

##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut prev = None;
        Self::postorder(root.take(), &mut prev);
        *root = prev;
    }

    fn postorder(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            let left = r.borrow_mut().left.take();
            let right = r.borrow_mut().right.take();
            Self::postorder(right, prev);
            Self::postorder(left, prev);
            r.borrow_mut().right = prev.take();
            *prev = Some(r);
        }
    }
}
```

`后序遍历`
