### [420. 强密码检验器](https://leetcode.cn/problems/strong-password-checker/)


如果一个密码满足下述所有条件，则认为这个密码是强密码：
- 由至少 6 个，至多 20 个字符组成。
- 至少包含 一个小写 字母，一个大写 字母，和 一个数字 。
- 同一字符 不能 连续出现三次 (比如 "...aaa..." 是不允许的, 但是 "...aa...a..." 如果满足其他条件也可以算是强密码)。
给你一个字符串 password ，返回 将 password 修改到满足强密码条件需要的最少修改步数。如果 password 已经是强密码，则返回 0 。

在一步修改操作中，你可以：

- 插入一个字符到 password ，
- 从 password 中删除一个字符，或
- 用另一个字符来替换 password 中的某个字符。


##### 示例 1：
```
输入：password = "a"
输出：5
```

##### 示例 2：
```
输入：password = "aA1"
输出：3
```

##### 示例 3：
```
输入：password = "1337C0d3"
输出：0
```

##### 提示：
- 1 <= password.length <= 50
- password 由字母、数字、点 '.' 或者感叹号 '!'

##### 题解：
```rust
impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let mut miss_types = 3;

        if password.chars().any(|c| c.is_lowercase()) {
            miss_types -= 1;
        }

        if password.chars().any(|c| c.is_uppercase()) {
            miss_types -= 1;
        }

        if password.chars().any(|c| c.is_digit(10)) {
            miss_types -= 1;
        }

        let n = password.len();
        let s = password.as_bytes();
        let (mut a, mut b, mut r) = (0, 0, 0);
        let mut i = 2;

        while i < n {
            if s[i] == s[i-1] && s[i] == s[i-2] {
                let mut tmp = 3;

                while i + 1 < n && s[i] == s[i+1] {
                    tmp += 1;
                    i += 1;
                }

                if tmp % 3 == 0 {
                    a += 1;
                } else if tmp % 3 == 1 {
                    b += 1;
                }

                r += tmp / 3;
            }

            i += 1;
        }

        if n < 6 {
            return miss_types.max(6 - n as i32);
        }

        if n <= 20 {
            return miss_types.max(r);
        }

        let d = n as i32 - 20;
        r -= a.min(d);

        if d > a {
            r -= b.min((d - a) / 2);
        }

        if d > a + 2 * b {
            r -= (d - a - 2 * b) / 3;
        }

        d + r.max(miss_types)
    }
}
```
