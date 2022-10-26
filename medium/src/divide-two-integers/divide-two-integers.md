### [29. 两数相除](https://leetcode.cn/problems/divide-two-integers/)

给定两个整数，被除数 dividend 和除数 divisor。将两数相除，要求不使用乘法、除法和 mod 运算符。

返回被除数 dividend 除以除数 divisor 得到的商。

整数除法的结果应当截去（truncate）其小数部分，例如：truncate(8.345) = 8 以及 truncate(-2.7335) = -2

 

##### 示例 1:
```
输入: dividend = 10, divisor = 3
输出: 3
解释: 10/3 = truncate(3.33333..) = truncate(3) = 3
```

##### 示例 2:
```
输入: dividend = 7, divisor = -3
输出: -2
解释: 7/-3 = truncate(-2.33333..) = -2
```

##### 提示：
- 被除数和除数均为 32 位有符号整数。
- 除数不为 0。
- 假设我们的环境只能存储32 位有符号整数，其数值范围是 [−2<sup>31</sup>,  2<sup>31</sup> − 1]。本题中，如果除法结果溢出，则返回 2<sup>31</sup> − 1

##### 题解：
```rust
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let is_neg = (dividend > 0) != (divisor > 0);
        let mut dividend = (dividend as i64).abs();
        let mut divisor = (divisor as i64).abs();
        let mut ans = 0;
        let mut cnt = 1;

        while (divisor << 1) <= dividend {
            cnt <<= 1;
            divisor <<= 1;
        }

        while cnt > 0 {
            if dividend >= divisor {
                ans += cnt;
                dividend -= divisor;
            }

            divisor >>= 1;
            cnt >>= 1;
        }

        if is_neg {
            -ans as i32
        } else {
            ans.min(i32::MAX as i64) as i32
        }
    }
}
```
