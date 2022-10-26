### [22. 括号生成](https://leetcode.cn/problems/generate-parentheses/)

数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。

##### 示例 1：
```
输入：n = 3
输出：["((()))","(()())","(())()","()(())","()()()"]
```

##### 示例 2：
```
输入：n = 1
输出：["()"]
```

##### 提示：

- 1 <= n <= 8

##### 题解：
```rust
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = vec![];

        Self::dfs(&mut ans, &mut String::new(), n, 0, 0);

        ans
    }

    fn dfs(ans: &mut Vec<String>, selected: &mut String, n: i32, left: i32, right: i32) {
        if selected.len() == (2 * n as usize) {
            ans.push(selected.clone());
            return;
        }

        if left < n {
            selected.push('(');
            Self::dfs(ans, selected, n , left + 1, right);
            selected.pop();
        }

        if right < left {
            selected.push(')');
            Self::dfs(ans, selected, n , left, right + 1);
            selected.pop();
        }
    }
}
```

`回溯`
