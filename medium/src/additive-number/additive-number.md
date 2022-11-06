### [306. 累加数](https://leetcode.cn/problems/additive-number/)
累加数 是一个字符串，组成它的数字可以形成累加序列。

一个有效的 累加序列 必须 至少 包含 3 个数。除了最开始的两个数以外，序列中的每个后续数字必须是它之前两个数字之和。

给你一个只包含数字 '0'-'9' 的字符串，编写一个算法来判断给定输入是否是 累加数 。如果是，返回 true ；否则，返回 false 。

说明：累加序列里的数，除数字 0 之外，不会 以 0 开头，所以不会出现 1, 2, 03 或者 1, 02, 3 的情况。



##### 示例 1：
```
输入："112358"
输出：true
解释：累加序列为: 1, 1, 2, 3, 5, 8 。1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8
```

##### 示例 2：
```
输入："199100199"
输出：true
解释：累加序列为: 1, 99, 100, 199。1 + 99 = 100, 99 + 100 = 199
```

##### 提示：
- 1 <= num.length <= 35
- num 仅由数字（0 - 9）组成


##### 进阶：
- 你计划如何处理由过大的整数输入导致的溢出?

##### 题解：
```rust
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let n = num.len();
        let mut ans = false;
        
        Self::dfs(&num, &mut Vec::new(), &mut ans, n, 0);

        ans
    }

    fn dfs(num: &str, selected: &mut Vec<u64>, ans: &mut bool, n: usize, begin: usize) {
        if begin == n {
            if selected.len() >= 3 {
                *ans = true;
            }

            return;
        }

        for i in begin+1..=n {
            if &num[begin..=begin] == "0" && i > begin + 1 {
                return;
            }

            if let Ok(v) = num[begin..i].parse::<u64>() {
                if selected.len() < 2 {
                    selected.push(v);
                    Self::dfs(num, selected, ans, n, i);
                    selected.pop();
                } else {
                    if selected[selected.len() - 2] + selected[selected.len() - 1] == v {
                        selected.push(v);
                        Self::dfs(num, selected, ans, n, i);
                        selected.pop();
                    }
                }
            }
        }
    }
}
```
