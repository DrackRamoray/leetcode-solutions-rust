### 366. 寻找二叉树的叶子节点
给你一棵二叉树，请按以下要求的顺序收集它的全部节点：

1. 依次从左到右，每次收集并删除所有的叶子节点
2. 重复如上过程直到整棵树为空


##### 示例:
```
输入: [1,2,3,4,5]

          1
         / \
        2   3
       / \     
      4   5    

输出: [[4,5,3],[2],[1]]
```

##### 解释:

1. 删除叶子节点 [4,5,3] ，得到如下树结构：
```
          1
         / 
        2          
```

2. 现在删去叶子节点 [2] ，得到如下树结构：
```
          1          
```

3. 现在删去叶子节点 [1] ，得到空树：
```
          []         
```

##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(&root, &mut ans);

        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<Vec<i32>>) -> i32 {
        if let Some(r) = root {
            let left = Self::dfs(&r.borrow().left, ans);
            let right = Self::dfs(&r.borrow().right, ans);
            let d = left.max(right) + 1;

            if ans.len() <= d as usize {
                ans.push(vec![]);
            }

            ans[d as usize].push(r.borrow().val);

            d
        } else {
            -1
        }
    }
}
```
