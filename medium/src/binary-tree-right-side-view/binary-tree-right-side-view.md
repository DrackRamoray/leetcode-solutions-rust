### [199. 二叉树的右视图](https://leetcode.cn/problems/binary-tree-right-side-view/)
给定一个二叉树的 根节点 root，想象自己站在它的右侧，按照从顶部到底部的顺序，返回从右侧所能看到的节点值。



##### 示例 1:
![img.png](img.png)
```
输入: [1,2,3,null,5,null,4]
输出: [1,3,4]
```

##### 示例 2:
```
输入: [1,null,3]
输出: [1,3]
```

##### 示例 3:
```
输入: []
输出: []
```

##### 提示:
- 二叉树的节点个数的范围是 [0,100]
- -100 <= Node.val <= 100 

##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut queue = vec![];
        
        if let Some(r) = root {
            queue.push(r);

            while queue.len() > 0 {
                let mut tmp = vec![];

                for i in 0..queue.len() {
                    if let Some(left) = queue[i].borrow_mut().left.take() {
                        tmp.push(left);
                    }

                    if let Some(right) = queue[i].borrow_mut().right.take() {
                        tmp.push(right);
                    }
                }

                ans.push(queue[queue.len()-1].borrow().val);
                queue = tmp;
            }
        }

        ans
    }
}
```
