### [7. 整数反转](https://leetcode.cn/problems/reverse-integer/)

给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。

如果反转后整数超过 32 位的有符号整数的范围 [−231,  231 − 1] ，就返回 0。

假设环境不允许存储 64 位整数（有符号或无符号）。

##### 示例 1：
```
输入：x = 123
输出：321
```

##### 示例 2：
```
输入：x = -123
输出：-321
```

##### 示例 3：
```
输入：x = 120
输出：21
```

##### 示例 4：
```
输入：x = 0
输出：0
```

##### 提示：
- -2<sup>31</sup> <= x <= 2<sup>31</sup> - 1

##### 题解：
```rust
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut ans = 0;

        while num != 0 {
            let rest = num % 10;

            if ans > i32::MAX / 10 || (ans == i32::MAX / 10 && rest > 7) {
                return 0;
            }

            if ans < i32::MIN / 10 || (ans == i32::MIN / 10 && rest < -8) {
                return 0;
            }

            ans = ans * 10 + rest;
            num = num / 10;
        }

        ans
    }
}
```

`数学`
