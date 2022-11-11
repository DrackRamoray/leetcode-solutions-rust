### [397. 整数替换](https://leetcode.cn/problems/integer-replacement/)
给定一个正整数 n ，你可以做如下操作：

- 如果 n 是偶数，则用 n / 2替换 n 。
- 如果 n 是奇数，则可以用 n + 1或n - 1替换 n 。

返回 n 变为 1 所需的 最小替换次数 。



##### 示例 1：
```
输入：n = 8
输出：3
解释：8 -> 4 -> 2 -> 1
```

##### 示例 2：
```
输入：n = 7
输出：4
解释：7 -> 8 -> 4 -> 2 -> 1
或 7 -> 6 -> 3 -> 2 -> 1
```

##### 示例 3：
```
输入：n = 4
输出：2
```

##### 提示：
- 1 <= n <= 2<sup>31</sup> - 1

##### 题解：
```rust
use std::collections::HashMap;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        Self::dfs(n, &mut HashMap::new())
    }

    fn dfs(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n == 1 {
            return 0;
        }

        if let Some(&v) = memo.get(&n) {
            return v;
        }

        let times = if n % 2 == 0 {
            1 + Self::dfs(n / 2, memo)
        } else {
            2 + Self::dfs(n / 2, memo).min(Self::dfs(n / 2 + 1, memo))
        };

        memo.insert(n, times);

        times
    }
}
```
