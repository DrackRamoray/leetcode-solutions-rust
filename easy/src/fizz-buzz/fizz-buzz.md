### [412. Fizz Buzz](https://leetcode.cn/problems/fizz-buzz/)
给你一个整数 n ，找出从 1 到 n 各个整数的 Fizz Buzz 表示，并用字符串数组 answer（下标从 1 开始）返回结果，其中：

- answer[i] == "FizzBuzz" 如果 i 同时是 3 和 5 的倍数。
- answer[i] == "Fizz" 如果 i 是 3 的倍数。
- answer[i] == "Buzz" 如果 i 是 5 的倍数。
- answer[i] == i （以字符串形式）如果上述条件全不满足。


##### 示例 1：
```
输入：n = 3
输出：["1","2","Fizz"]
```

##### 示例 2：
```
输入：n = 5
输出：["1","2","Fizz","4","Buzz"]
```

##### 示例 3：
```
输入：n = 15
输出：["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]
```

##### 提示：
- 1 <= n <= 10<sup>4</sup>

##### 题解：
```rust
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res = vec![];

        for i in 1..=n {
            let d3 = i % 3 == 0;
            let d5 = i % 5 == 0;
            if d3 && d5 {
                res.push("FizzBuzz".to_string());
            } else if d5 {
                res.push("Buzz".to_string());
            } else if d3 {
                res.push("Fizz".to_string())
            } else {
                res.push(i.to_string())
            }
        }

        res
    }
}
```
