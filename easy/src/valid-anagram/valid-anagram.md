### [242. 有效的字母异位词](https://leetcode.cn/problems/valid-anagram/)
给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。

注意：若 s 和 t 中每个字符出现的次数都相同，则称 s 和 t 互为字母异位词。



##### 示例 1:
```
输入: s = "anagram", t = "nagaram"
输出: true
```

##### 示例 2:
```
输入: s = "rat", t = "car"
输出: false
```

##### 提示:
- 1 <= s.length, t.length <= 5 * 10<sup>4</sup>
- s 和 t 仅包含小写字母


##### 进阶: 
- 如果输入字符串包含 unicode 字符怎么办？你能否调整你的解法来应对这种情况？

##### 题解：
```rust
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut vec = s.bytes().fold(vec![0;26], |mut acc, u| {
            acc[u as usize - b'a' as usize] += 1;
            acc
        });
        
        for u in t.bytes() {
            vec[u as usize - b'a' as usize] -= 1;

            if vec[u as usize - b'a' as usize] < 0 {
                return false;
            }
        }

        true
    }
}
```
