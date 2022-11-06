### [316. 去除重复字母](https://leetcode.cn/problems/remove-duplicate-letters/)
给你一个字符串 s ，请你去除字符串中重复的字母，使得每个字母只出现一次。需保证 返回结果的字典序最小（要求不能打乱其他字符的相对位置）。



##### 示例 1：
```
输入：s = "bcabc"
输出："abc"
```

##### 示例 2：
```
输入：s = "cbacdcbc"
输出："acdb"
```

##### 提示：
- 1 <= s.length <= 10<sup>4</sup>
- s 由小写英文字母组成

##### 题解：
```rust
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stack = vec![];
        let mut mp = vec![0;26];

        for b in s.bytes() {
            mp[(b - b'a') as usize] += 1;
        }

        let mut vis = vec![false;26];

        for b in s.bytes() {
            let u = (b - b'a') as usize;
            mp[u] -= 1;

            if !vis[u] {
                vis[u] = true;

                while let Some(&v) = stack.last() {
                    let u = (v - b'a') as usize;
                    if v > b && mp[u] > 0 {
                        vis[u] = false;
                        stack.pop();
                    } else {
                        break;
                    }
                }

                stack.push(b);
            }
        }

        stack.into_iter().map(|u| u as char).collect()
    }
}

```
