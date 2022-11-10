### [387. 字符串中的第一个唯一字符](https://leetcode.cn/problems/first-unique-character-in-a-string/)
给定一个字符串 s ，找到 它的第一个不重复的字符，并返回它的索引 。如果不存在，则返回 -1 。



##### 示例 1：
```
输入: s = "leetcode"
输出: 0
```

##### 示例 2:
```
输入: s = "loveleetcode"
输出: 2
```

##### 示例 3:
```
输入: s = "aabb"
输出: -1
```

##### 提示:
- 1 <= s.length <= 10<sup>5</sup>
- s 只包含小写字母

##### 题解：
```rust
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let ss = s.as_bytes();
        let cnt = ss.iter().fold(vec![0;26], |mut acc, &b| {
            acc[b as usize - b'a' as usize] += 1;
            acc
        });

        for i in 0..s.len() {
            if cnt[ss[i] as usize - b'a' as usize] == 1 {
                return i as i32;
            }
        }

        -1
    }
}

```
