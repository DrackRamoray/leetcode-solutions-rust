### [255. 验证前序遍历序列二叉搜索树](https://leetcode.cn/problems/verify-preorder-sequence-in-binary-search-tree/)

##### 题解：
```rust
impl Solution {
    pub fn verify_preorder(preorder: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut root_val = i32::MIN;

        for &num in preorder.iter() {
            if num < root_val {
                return false;
            }

            while stack.len() > 0 && stack[stack.len() - 1] < num {
                if let Some(v) = stack.pop() {
                    root_val = v;
                }
            }

            stack.push(num);
        }

        true
    }
}
```
