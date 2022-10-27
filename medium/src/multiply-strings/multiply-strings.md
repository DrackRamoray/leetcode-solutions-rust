### [43. 字符串相乘](https://leetcode.cn/problems/multiply-strings/)

给定两个以字符串形式表示的非负整数 num1 和 num2，返回 num1 和 num2 的乘积，它们的乘积也表示为字符串形式。

注意：不能使用任何内置的 BigInteger 库或直接将输入转换为整数。

##### 示例 1:
```
输入: num1 = "2", num2 = "3"
输出: "6"
```

##### 示例 2:
```
输入: num1 = "123", num2 = "456"
输出: "56088"
```

##### 提示：
- 1 <= num1.length, num2.length <= 200
- num1 和 num2 只能由数字组成。
- num1 和 num2 都不包含任何前导零，除了数字0本身。

##### 题解：
```rust
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let n = num1.len();
        let m = num2.len();
        let mut ans = vec![0;n+m];
        let n1 = num1.as_bytes();
        let n2 = num2.as_bytes();

        for i in (0..n).rev() {
            let a = n1[i] as i32 - 48;

            for j in (0..m).rev() {
                let b = n2[j] as i32 - 48;
                ans[i+j+1] += a * b;
            }
        }

        for i in (1..n+m).rev() {
            ans[i-1] += ans[i] / 10;
            ans[i] = ans[i] % 10;
        }

        let mut i = 0;

        while i < n+m-1 {
            if ans[i] == 0 {
                i += 1;
            } else {
                break;
            }
        }

        ans[i..].into_iter().map(|x| x.to_string()).collect::<String>()
    }
}
```

`字符串` `模拟`
