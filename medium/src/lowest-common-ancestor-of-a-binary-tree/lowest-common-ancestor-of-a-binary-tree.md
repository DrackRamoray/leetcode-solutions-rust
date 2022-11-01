### [236. 二叉树的最近公共祖先](https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/)

给定一个二叉树, 找到该树中两个指定节点的最近公共祖先。

百度百科中最近公共祖先的定义为：“对于有根树 T 的两个节点 p、q，最近公共祖先表示为一个节点 x，满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点也可以是它自己的祖先）。”



##### 示例 1：
![img_1.png](img_1.png)
```
输入：root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
输出：3
解释：节点 5 和节点 1 的最近公共祖先是节点 3 。
```

##### 示例 2：
![img.png](img.png)
```
输入：root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
输出：5
解释：节点 5 和节点 4 的最近公共祖先是节点 5 。因为根据定义最近公共祖先节点可以为节点本身。
```

##### 示例 3：
```
输入：root = [1,2], p = 1, q = 2
输出：1
```

##### 提示：
- 树中节点数目在范围 [2, 10<sup>5</sup>] 内。
- -10<sup>9</sup> <= Node.val <= 10<sup>9</sup>
- 所有 Node.val 互不相同 。
- p != q
- p 和 q 均存在于给定的二叉树中。

##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(mut root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if p == root || q == root {
            return root;
        }

        if let Some(r) = root {
            let left = Self::lowest_common_ancestor(r.borrow_mut().left.take(), p.clone(), q.clone());
            let right = Self::lowest_common_ancestor(r.borrow_mut().right.take(), p, q);

            if left.is_none() {
                return right;
            }

            if right.is_none() {
                return left;
            }

            Some(r)
        } else {
            None
        }
    }
}
```
