### [423. 从英文中重建数字](https://leetcode.cn/problems/reconstruct-original-digits-from-english/)
给你一个字符串 s ，其中包含字母顺序打乱的用英文单词表示的若干数字（0-9）。按 升序 返回原始的数字。



##### 示例 1：
```
输入：s = "owoztneoer"
输出："012"
```

##### 示例 2：
```
输入：s = "fviefuro"
输出："45"
```

##### 提示：
- 1 <= s.length <= 10<sup>5</sup>
- s[i] 为 ["e","g","f","i","h","o","n","s","r","u","t","w","v","x","z"] 这些字符之一
- s 保证是一个符合题目要求的字符串

##### 题解：
```rust
impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut mp = std::collections::HashMap::new();

        for c in s.chars() {
            *mp.entry(c).or_insert(0) += 1;
        }

        let mut cnt = [0;10];
        cnt[0] = *mp.get(&'z').unwrap_or(&0);
        cnt[2] = *mp.get(&'w').unwrap_or(&0);
        cnt[4] = *mp.get(&'u').unwrap_or(&0);
        cnt[6] = *mp.get(&'x').unwrap_or(&0);
        cnt[8] = *mp.get(&'g').unwrap_or(&0);
        cnt[3] = *mp.get(&'h').unwrap_or(&0) - cnt[8];
        cnt[5] = *mp.get(&'f').unwrap_or(&0) - cnt[4];
        cnt[7] = *mp.get(&'s').unwrap_or(&0) - cnt[6];
        cnt[1] = *mp.get(&'o').unwrap_or(&0) - cnt[0] - cnt[2] - cnt[4];
        cnt[9] = *mp.get(&'i').unwrap_or(&0) - cnt[5] - cnt[6] - cnt[8];

        let mut ans = String::new();

        for i in 0..10 {
            for _ in 0..cnt[i] {
                ans.push((i as u8 + '0' as u8) as char);
            }
        }

        ans
    }
}
```
