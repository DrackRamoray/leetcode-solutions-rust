### [402. 移掉 K 位数字](https://leetcode.cn/problems/remove-k-digits/)
给你一个以字符串表示的非负整数 num 和一个整数 k ，移除这个数中的 k 位数字，使得剩下的数字最小。请你以字符串形式返回这个最小的数字。


##### 示例 1 ：
```
输入：num = "1432219", k = 3
输出："1219"
解释：移除掉三个数字 4, 3, 和 2 形成一个新的最小的数字 1219 。
```

##### 示例 2 ：
```
输入：num = "10200", k = 1
输出："200"
解释：移掉首位的 1 剩下的数字为 200. 注意输出不能有任何前导零。
```

##### 示例 3 ：
```
输入：num = "10", k = 2
输出："0"
解释：从原数字移除所有的数字，剩余为空就是 0 。
```

##### 提示：
- 1 <= k <= num.length <= 10<sup>5</sup>
- num 仅由若干位数字（0 - 9）组成
- 除了 0 本身之外，num 不含任何前导零

##### 题解：
```rust
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let len = num.len();
        let mut k = k as usize;
        let mut pos = 1;
        let mut stack = vec![];
        let s = num.as_bytes();
        stack.push(s[0]);

        while pos < len {
            while k > 0 && stack.len() > 0 && stack[stack.len() - 1] > s[pos] {
                k -= 1;
                stack.pop();
            }

            stack.push(s[pos]);
            pos += 1;
        }

        let res = stack[0..(stack.len()-k)].iter().skip_while(|&&c| c == b'0').map(|&c| c as char).collect::<String>();

        if res.len() == 0 {
            return "0".into();
        }

        res
    }
}
```
