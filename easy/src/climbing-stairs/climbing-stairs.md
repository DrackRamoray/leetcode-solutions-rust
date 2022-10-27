### [70. 爬楼梯](https://leetcode.cn/problems/climbing-stairs/)

假设你正在爬楼梯。需要 n 阶你才能到达楼顶。

每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？

##### 示例 1：
```
输入：n = 2
输出：2
解释：有两种方法可以爬到楼顶。
1. 1 阶 + 1 阶
2. 2 阶
```

##### 示例 2：
```
输入：n = 3
输出：3
解释：有三种方法可以爬到楼顶。
1. 1 阶 + 1 阶 + 1 阶
2. 1 阶 + 2 阶
3. 2 阶 + 1 阶
```

##### 提示：
- 1 <= n <= 45

##### 题解：
```rust
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        let mut ans = 0;

        for i in 1..=n {
            ans = a + b;
            a = b;
            b = ans;
        }

        ans
    }
}
```

`动态规划`
