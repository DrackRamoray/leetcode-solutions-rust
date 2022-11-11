### [395. 至少有 K 个重复字符的最长子串](https://leetcode.cn/problems/longest-substring-with-at-least-k-repeating-characters/)
给你一个字符串 s 和一个整数 k ，请你找出 s 中的最长子串， 要求该子串中的每一字符出现次数都不少于 k 。返回这一子串的长度。



##### 示例 1：
```
输入：s = "aaabb", k = 3
输出：3
解释：最长子串为 "aaa" ，其中 'a' 重复了 3 次。
```

##### 示例 2：
```
输入：s = "ababbc", k = 2
输出：5
解释：最长子串为 "ababb" ，其中 'a' 重复了 2 次， 'b' 重复了 3 次。
```

##### 提示：
- 1 <= s.length <= 10<sup>4</sup>
- s 仅由小写英文字母组成
- 1 <= k <= 10<sup>5</sup>

##### 题解：
```rust
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::helper(s.as_bytes(), 0, s.len(), k as usize)
    }

    fn helper(s: &[u8], begin: usize, end: usize, k: usize) -> i32 {
        if end < k {
            return 0;
        }

        let mut cnt = vec![0;26];

        for i in begin..end {
            cnt[s[i] as usize - 'a' as usize] += 1;
        }

        for i in begin..end {
            if cnt[s[i] as usize - 'a' as usize] >= k {
                continue;
            }

            let mut j = i + 1;

            while j < end && cnt[s[j] as usize - 'a' as usize] < k {
                j += 1;
            }

            return (Self::helper(s, begin, i, k)).max(Self::helper(s, j, end, k));
        }

        end as i32 - begin as i32
    }
}
```
