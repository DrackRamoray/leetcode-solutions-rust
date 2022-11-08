### [358. K 距离间隔重排字符串](https://leetcode.cn/problems/rearrange-string-k-distance-apart/)
给你一个非空的字符串 s 和一个整数 k ，你要将这个字符串 s 中的字母进行重新排列，使得重排后的字符串中相同字母的位置间隔距离 至少 为 k 。如果无法做到，请返回一个空字符串 ""。



##### 示例 1：
```
输入: s = "aabbcc", k = 3
输出: "abcabc"
解释: 相同的字母在新的字符串中间隔至少 3 个单位距离。
```

##### 示例 2:
```
输入: s = "aaabc", k = 3
输出: ""
解释: 没有办法找到可能的重排结果。
```

##### 示例 3:
```
输入: s = "aaadbbcc", k = 2
输出: "abacabcd"
解释: 相同的字母在新的字符串中间隔至少 2 个单位距离。
```

##### 提示：
- 1 <= s.length <= 3 * 10<sup>5</sup>
- s 仅由小写英文字母组成
- 0 <= k <= s.length

##### 题解：
```rust
use std::collections::BinaryHeap;
use std::collections::VecDeque;

impl Solution {
    pub fn rearrange_string(s: String, k: i32) -> String {
        if k == 0 {
            return s;
        }

        let mut cnt = vec![0;26];
        let mut queue = BinaryHeap::new();
        let mut remain = VecDeque::new();
        
        for &u in s.as_bytes() {
            cnt[u as usize - 'a' as usize] += 1;
        }

        for i in 0..cnt.len() {
            if cnt[i] > 0 {
                queue.push((cnt[i], (i as u8 + b'a')));
            }
        }

        let mut ans = String::new();

        while let Some((v, u)) = queue.pop() {
            ans.push(u as char);
            remain.push_back((v-1, u));

            if remain.len() == k as usize {
                if let Some(&(t, k)) = remain.front() {
                    if t > 0 {
                        queue.push((t, k));
                    }
                }
                remain.pop_front();
            }
        }

        if ans.len() < s.len() {
            "".to_string()
        } else {
            ans
        }
    }
}
```
