### [105. 从前序与中序遍历序列构造二叉树](https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)

给定两个整数数组 preorder 和 inorder ，其中 preorder 是二叉树的先序遍历， inorder 是同一棵树的中序遍历，请构造二叉树并返回其根节点。



##### 示例 1:
![img.png](img.png)
```
输入: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
输出: [3,9,20,null,null,15,7]
```

##### 示例 2:
```
输入: preorder = [-1], inorder = [-1]
输出: [-1]
```

##### 提示:
- 1 <= preorder.length <= 3000
- inorder.length == preorder.length
- -3000 <= preorder[i], inorder[i] <= 3000
- preorder 和 inorder 均 无重复 元素
- inorder 均出现在 preorder
- preorder 保证 为二叉树的前序遍历序列
- inorder 保证 为二叉树的中序遍历序列

##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&preorder, &inorder)
    }

    pub fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));

        let mut index = 0;

        while index < inorder.len() {
            if inorder[index] == preorder[0] {
                break;
            }
            index += 1;
        }

        root.borrow_mut().left = Self::build(&preorder[1..=index], &inorder[0..index]);
        root.borrow_mut().right = Self::build(&preorder[index+1..], &inorder[index+1..]);

        Some(root)
    }
}
```

`递归`
