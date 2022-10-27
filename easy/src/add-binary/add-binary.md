### [67. 二进制求和](https://leetcode.cn/problems/add-binary/)

给你两个二进制字符串 a 和 b ，以二进制字符串的形式返回它们的和。

##### 示例 1：
```
输入:a = "11", b = "1"
输出："100"
```

##### 示例 2：
```
输入：a = "1010", b = "1011"
输出："10101"
```

##### 提示：
- 1 <= a.length, b.length <= 10<sup>4</sup>
- a 和 b 仅由字符 '0' 或 '1' 组成
- 字符串如果不是 "0" ，就不含前导零

##### 题解：
```rust
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let n = a.len();
        let m = b.len();
        let c = if n >= m { n + 1 } else { m + 1 };
        let mut ans = vec![0;c];
        let mut aa = a.bytes().rev();
        let mut bb = b.bytes().rev();
        let mut carry = 0;

        for i in (0..c).rev() {
            match (aa.next(), bb.next()) {
                (Some(ua), Some(ub)) => {
                    let tmp = (ua as i32) + (ub as i32) - 96 + carry;
                    ans[i] = tmp % 2;
                    carry = tmp / 2;
                },
                (Some(ua), None) => {
                    let tmp = (ua as i32) - 48 + carry;
                    ans[i] = tmp % 2;
                    carry = tmp / 2;
                },
                (None, Some(ub)) => {
                    let tmp = (ub as i32) - 48 + carry;
                    ans[i] = tmp % 2;
                    carry = tmp / 2;
                },
                _ => {
                    if carry != 0 {
                        ans[i] = carry;
                    }
                },
            }
        }

        let mut skip = 0;

        for i in 0..c-1 {
            if ans[i] == 0 {
                skip += 1;
            } else {
                break;
            }
        }

        ans[skip..].into_iter().map(|x| x.to_string()).collect::<String>()
    }
}
```

`字符串`
