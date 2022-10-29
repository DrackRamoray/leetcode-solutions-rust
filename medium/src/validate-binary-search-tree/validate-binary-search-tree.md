### 98. 验证二叉搜索树
给你一个二叉树的根节点 root ，判断其是否是一个有效的二叉搜索树。

有效 二叉搜索树定义如下：

- 节点的左子树只包含 小于 当前节点的数。
- 节点的右子树只包含 大于 当前节点的数。
- 所有左子树和右子树自身必须也是二叉搜索树。


##### 示例 1：
![img.png](img.png)
```
输入：root = [2,1,3]
输出：true
```

##### 示例 2：
![img_1.png](img_1.png)
```
输入：root = [5,1,4,null,null,3,6]
输出：false
解释：根节点的值是 5 ，但是右子节点的值是 4 。
```

##### 提示：
- 树中节点数目范围在[1, 104] 内
- -2<sup>31</sup> <= Node.val <= 2<sup>31</sup> - 1


##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_valid_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = vec![];
        let mut prev = i64::MIN;
        
        while stack.len() > 0 || root.is_some() {
            while let Some(r) = root {
                root = r.borrow_mut().left.take();
                stack.push(r);
            }

            if let Some(r) = stack.pop() {
                if (r.borrow().val as i64) <= prev {
                    return false;
                } 

                prev = r.borrow().val as i64;
                root = r.borrow_mut().right.take();
            }
        }

        return true;
    }
}
```

`深度优先搜索`
