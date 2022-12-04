### [438. 找到字符串中所有字母异位词](https://leetcode.cn/problems/find-all-anagrams-in-a-string/)
给定两个字符串 s 和 p，找到 s 中所有 p 的 异位词 的子串，返回这些子串的起始索引。不考虑答案输出的顺序。

异位词 指由相同字母重排列形成的字符串（包括相同的字符串）。



##### 示例 1:
```
输入: s = "cbaebabacd", p = "abc"
输出: [0,6]
解释:
起始索引等于 0 的子串是 "cba", 它是 "abc" 的异位词。
起始索引等于 6 的子串是 "bac", 它是 "abc" 的异位词。
```

##### 示例 2:
```
输入: s = "abab", p = "ab"
输出: [0,1,2]
解释:
起始索引等于 0 的子串是 "ab", 它是 "ab" 的异位词。
起始索引等于 1 的子串是 "ba", 它是 "ab" 的异位词。
起始索引等于 2 的子串是 "ab", 它是 "ab" 的异位词。
```

##### 提示:
- 1 <= s.length, p.length <= 3 * 10<sup>4</sup>
- s 和 p 仅包含小写字母

##### 题解：
```rust
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let n = s.len();
        let m = p.len();
        let ss = s.as_bytes();
        let pp = p.as_bytes();
        let mut cnt_s = vec![0;26];
        let mut cnt_p = vec![0;26];
        let mut ans = vec![];

        for i in 0..m {
            cnt_p[pp[i] as usize - 97] += 1;
        }

        for i in 0..n {
            cnt_s[ss[i] as usize - 97] += 1;

            if i >= m {
                cnt_s[ss[i-m] as usize - 97] -= 1;
            }

            if i >= m - 1 && cnt_s == cnt_p {
                ans.push(i as i32 - m as i32 + 1);
            }
        }

        ans
    }
}
```
