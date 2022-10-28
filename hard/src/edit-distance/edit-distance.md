### [72. 编辑距离](https://leetcode.cn/problems/edit-distance/)

给你两个单词 word1 和 word2， 请返回将 word1 转换成 word2 所使用的最少操作数  。

你可以对一个单词进行如下三种操作：

 - 插入一个字符
 - 删除一个字符
 - 替换一个字符

##### 示例 1：
```
输入：word1 = "horse", word2 = "ros"
输出：3
解释：
horse -> rorse (将 'h' 替换为 'r')
rorse -> rose (删除 'r')
rose -> ros (删除 'e')
```

##### 示例 2：
```
输入：word1 = "intention", word2 = "execution"
输出：5
解释：
intention -> inention (删除 't')
inention -> enention (将 'i' 替换为 'e')
enention -> exention (将 'n' 替换为 'x')
exention -> exection (将 'n' 替换为 'c')
exection -> execution (插入 'u')
```

##### 提示：
- 0 <= word1.length, word2.length <= 500
- word1 和 word2 由小写英文字母组成

##### 题解：
```rust
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let n = word1.len();
        let m = word2.len();
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let mut dp = vec![vec![0;m+1];n+1];

        for i in 1..=n {
            dp[i][0] = i;
        }

        for j in 1..=m {
            dp[0][j] = j;
        }

        for i in 1..=n {
            for j in 1..=m {
                if w1[i-1] == w2[j-1] {
                    dp[i][j] = dp[i-1][j-1];
                } else {
                    dp[i][j] = dp[i-1][j-1].min(dp[i-1][j]).min(dp[i][j-1]) + 1;
                }
            }
        }

        dp[n][m] as i32
    }
}
```

`动态规划`
