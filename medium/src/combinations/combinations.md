### [77. 组合](https://leetcode.cn/problems/combinations/)

给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。

你可以按 任何顺序 返回答案。



##### 示例 1：
```
输入：n = 4, k = 2
输出：
[
[2,4],
[3,4],
[2,3],
[1,2],
[1,3],
[1,4],
]
```

##### 示例 2：
```
输入：n = 1, k = 1
输出：[[1]]
```

##### 提示：
- 1 <= n <= 20
- 1 <= k <= n

##### 题解：
```rust
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();

        Self::dfs(n as usize, k as usize, &mut ans, &mut Vec::new(), 1);

        ans
    }

    fn dfs(n: usize, k: usize, ans: &mut Vec<Vec<i32>>, selected: &mut Vec<i32>, cur: usize) {
        if selected.len() + (n - cur + 1) < k {
            return;
        }
        
        if selected.len() == k {
            ans.push(selected.to_vec());
            return;
        }

        for i in cur..=n {
            selected.push(i as i32);
            Self::dfs(n, k, ans, selected, i + 1);
            selected.pop();
        }
    }
}
```

`回溯`
