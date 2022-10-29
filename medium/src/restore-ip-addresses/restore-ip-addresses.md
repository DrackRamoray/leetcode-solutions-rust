### [93. 复原 IP 地址](https://leetcode.cn/problems/restore-ip-addresses/)

有效 IP 地址 正好由四个整数（每个整数位于 0 到 255 之间组成，且不能含有前导 0），整数之间用 '.' 分隔。

- 例如："0.1.2.201" 和 "192.168.1.1" 是 有效 IP 地址，但是 "0.011.255.245"、"192.168.1.312" 和 "192.168@1.1" 是 无效 IP 地址。
给定一个只包含数字的字符串 s ，用以表示一个 IP 地址，返回所有可能的有效 IP 地址，这些地址可以通过在 s 中插入 '.' 来形成。你 不能 重新排序或删除 s 中的任何数字。你可以按 任何 顺序返回答案。



##### 示例 1：
```
输入：s = "25525511135"
输出：["255.255.11.135","255.255.111.35"]
```

##### 示例 2：
```
输入：s = "0000"
输出：["0.0.0.0"]
```

##### 示例 3：
```
输入：s = "101023"
输出：["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
```

##### 提示：
- 1 <= s.length <= 20
- s 仅由数字组成

##### 题解:
```rust
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ans = vec![];

        Self::dfs(&mut ans, &mut [0;4], s.as_bytes(), 0, 0);

        ans
    }

    fn dfs(ans: &mut Vec<String>, selected: &mut [u8;4], s: &[u8], cur: usize, start: usize) {
        if cur == 4 {
            if start == s.len() {
                ans.push(selected.iter().map(|num| num.to_string()).collect::<Vec<String>>().join("."));
            }
            return;
        }

        if start == s.len() {
            return;
        }

        if s[start] == b'0' {
            selected[cur] = 0;
            Self::dfs(ans, selected, s, cur + 1, start + 1);
        }

        let mut addr = 0_i32;

        for i in start..s.len() {
            addr = addr * 10 + s[i] as i32 - b'0' as i32;

            if addr > 0 && addr < 256 {
                selected[cur] = addr as u8;
                Self::dfs(ans, selected, s, cur + 1, i + 1);
            } else {
                break;
            }
        }
    }
}
```

`回溯`
