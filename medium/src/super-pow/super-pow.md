### [372. 超级次方](https://leetcode.cn/problems/super-pow/)
你的任务是计算 ab 对 1337 取模，a 是一个正整数，b 是一个非常大的正整数且会以数组形式给出。



##### 示例 1：
```
输入：a = 2, b = [3]
输出：8
```

##### 示例 2：
```
输入：a = 2, b = [1,0]
输出：1024
```

##### 示例 3：
```
输入：a = 1, b = [4,3,3,8,5,2]
输出：1
```

##### 示例 4：
```
输入：a = 2147483647, b = [2,0,0]
输出：1198
```

##### 提示：
- 1 <= a <= 2<sup>31</sup> - 1
- 1 <= b.length <= 2000
- 0 <= b[i] <= 9
- b 不含前导 0

##### 题解：
```rust
impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let mut ans = 1;

        for v in b {
            ans = Self::power(ans, 10) * Self::power(a as i64, v as i64) % 1337;
        }

        ans as i32
    }

    fn power(mut x: i64, mut n: i64) -> i64 {
        let mut ans = 1;

        while n > 0 {
            if n % 2 == 1 {
                ans = ans * x % 1337;
            }

            x = x * x % 1337;
            n /= 2;
        }

        ans
    } 
}
```
