### [60. 排列序列](https://leetcode.cn/problems/permutation-sequence/)

给出集合 [1,2,3,...,n]，其所有元素共有 n! 种排列。

按大小顺序列出所有排列情况，并一一标记，当 n = 3 时, 所有排列如下：

1. "123"
2. "132"
3. "213"
4. "231"
5. "312"
6. "321"

给定 n 和 k，返回第 k 个排列。

 

##### 示例 1：
```
输入：n = 3, k = 3
输出："213"
```

##### 示例 2：
```
输入：n = 4, k = 9
输出："2314"
```

##### 示例 3：
```
输入：n = 3, k = 1
输出："123"
```

##### 提示：
- 1 <= n <= 9
- 1 <= k <= n!

##### 题解：
 ```rust
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut fact = vec![0;n];
        fact[0] = 1;

        for i in 1..n {
            fact[i] = fact[i-1] * i;
        }

        let mut k = k as usize - 1;
        let mut ans = vec![];
        let mut valid = vec![1;n + 1];

        for i in 1..=n {
            let mut order = k / fact[n - i] + 1;

            for j in 1..=n {
                order -= valid[j];

                if order == 0 {
                    ans.push(j);
                    valid[j] = 0;
                    break;
                }
            }

            k %= fact[n-i];
        }

        ans.into_iter().map(|v| (v as u8 + b'0') as char).collect()
    }
}
```
