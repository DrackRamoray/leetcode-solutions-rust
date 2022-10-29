### [101. 对称二叉树](https://leetcode.cn/problems/symmetric-tree/)

给你一个二叉树的根节点 root ， 检查它是否轴对称。



##### 示例 1：
![img.png](img.png)
```
输入：root = [1,2,2,3,4,4,3]
输出：true
```

##### 示例 2：
![img_1.png](img_1.png)
```
输入：root = [1,2,2,null,3,null,3]
输出：false
```

##### 提示：
- 树中节点数目在范围 [1, 1000] 内
- -100 <= Node.val <= 100


##### 进阶：
- 你可以运用递归和迭代两种方法解决这个问题吗？

##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(mut r) = root {
            let mut queue = VecDeque::new();
            queue.push_back(r.borrow_mut().left.take());
            queue.push_back(r.borrow_mut().right.take());

            while queue.len() > 0 {
                match (queue.pop_front(), queue.pop_front()) {
                    (Some(Some(n1)), Some(Some(n2))) => {
                        if n1.borrow().val != n2.borrow().val {
                            return false;
                        }

                        queue.push_back(n1.borrow_mut().left.take());
                        queue.push_back(n2.borrow_mut().right.take());

                        queue.push_back(n1.borrow_mut().right.take());
                        queue.push_back(n2.borrow_mut().left.take());
                    },
                    (Some(None), Some(None)) => {},
                    (None, None) => {},
                    _ => {
                        return false;
                    }
                }
            }

            true
        } else {
            true
        }
    }
}
```

`队列`
