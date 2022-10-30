### [129. 求根节点到叶节点数字之和](https://leetcode.cn/problems/sum-root-to-leaf-numbers/)

给你一个二叉树的根节点 root ，树中每个节点都存放有一个 0 到 9 之间的数字。
每条从根节点到叶节点的路径都代表一个数字：

- 例如，从根节点到叶节点的路径 1 -> 2 -> 3 表示数字 123 。

计算从根节点到叶节点生成的 所有数字之和 。

叶节点 是指没有子节点的节点。



##### 示例 1：
![img_1.png](img_1.png)
```
输入：root = [1,2,3]
输出：25
解释：
从根到叶子节点路径 1->2 代表数字 12
从根到叶子节点路径 1->3 代表数字 13
因此，数字总和 = 12 + 13 = 25
```

##### 示例 2：
![img.png](img.png)
```
输入：root = [4,9,0,5,1]
输出：1026
解释：
从根到叶子节点路径 4->9->5 代表数字 495
从根到叶子节点路径 4->9->1 代表数字 491
从根到叶子节点路径 4->0 代表数字 40
因此，数字总和 = 495 + 491 + 40 = 1026
```

##### 提示：
- 树中节点的数目在范围 [1, 1000] 内
- 0 <= Node.val <= 9
- 树的深度不超过 10

##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(root, 0)
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if let Some(r) = root {
            let ans = sum * 10 + r.borrow().val;

            if r.borrow().left.is_none() && r.borrow().right.is_none() {
                ans
            } else {
                Self::helper(r.borrow().left.clone(), ans) + Self::helper(r.borrow().right.clone(), ans)
            }
        } else {
            0
        }
    }
}
```

`深度优先搜索`
