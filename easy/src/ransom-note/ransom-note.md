### [383. 赎金信](https://leetcode.cn/problems/ransom-note/)
给你两个字符串：ransomNote 和 magazine ，判断 ransomNote 能不能由 magazine 里面的字符构成。

如果可以，返回 true ；否则返回 false 。

magazine 中的每个字符只能在 ransomNote 中使用一次。



##### 示例 1：
```
输入：ransomNote = "a", magazine = "b"
输出：false
```

##### 示例 2：
```
输入：ransomNote = "aa", magazine = "ab"
输出：false
```

##### 示例 3：
```
输入：ransomNote = "aa", magazine = "aab"
输出：true
```

##### 提示：
- 1 <= ransomNote.length, magazine.length <= 10<sup>5</sup>
- ransomNote 和 magazine 由小写英文字母组成

##### 题解：
```rust
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut cnt = vec![0;26];

        for ch in magazine.chars() {
            cnt[ch as usize - 97] += 1;
        }

        for ch in ransom_note.chars() {
            cnt[ch as usize - 97] -= 1;

            if cnt[ch as usize - 97] < 0 {
                return false;
            }
        }

        true
    }
}
```
