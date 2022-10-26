### [14. 最长公共前缀](https://leetcode.cn/problems/longest-common-prefix/)

编写一个函数来查找字符串数组中的最长公共前缀。

如果不存在公共前缀，返回空字符串 `""`。

##### 示例 1：
```
输入：strs = ["flower","flow","flight"]
输出："fl"
```

##### 示例 2：
```
输入：strs = ["dog","racecar","car"]
输出：""
解释：输入不存在公共前缀。
```

##### 提示：
- 1 <= strs.length <= 200
- 0 <= strs[i].length <= 200
- strs[i] 仅由小写英文字母组成

##### 题解：
```rust
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let n = strs.len();

        if n < 1 {
            return "".to_owned();
        }

        let mut prefix = strs[0].as_str();

        for i in 1..n {
            prefix = Solution::lcp(prefix, &strs[i]);
        }

        prefix.into()
    }

    fn lcp<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        let len = str1.len().min(str2.len());
        let mut ans = 0_usize;

        while ans < len && &str1[ans..ans+1] == &str2[ans..ans+1] {
            ans += 1;
        }

        return &str1[0..ans]
    }
}
```

`最长公共子字符串`
