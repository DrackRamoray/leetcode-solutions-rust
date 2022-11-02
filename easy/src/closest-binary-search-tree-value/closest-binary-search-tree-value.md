### [270. 最接近的二叉搜索树值](https://leetcode.cn/problems/closest-binary-search-tree-value/)
给定一个不为空的二叉搜索树和一个目标值 target，请在该二叉搜索树中找到最接近目标值 target 的数值。

##### 注意：
- 给定的目标值 target 是一个浮点数
- 题目保证在该二叉搜索树中只会存在一个最接近目标值的数

##### 示例：
```
输入: root = [4,2,5,1,3]，目标值 target = 3.714286

    4
   / \
  2   5
 / \
1   3

输出: 4
```

##### 题解:
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        if let Some(ref rf) = root {
            let mut val = rf.borrow().val;
            let mut closet = val;
            let mut r = root;

            while let Some(ref node) = r {
                val = node.borrow().val;
                closet = if (val as f64 - target).abs() < (closet as f64 - target).abs() { val } else { closet };
                r = if target < val as f64 { node.borrow_mut().left.take() } else { node.borrow_mut().right.take() };
            }

            closet
        } else {
            -1
        }
    }
}
```
