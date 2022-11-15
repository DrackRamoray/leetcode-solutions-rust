### [415. 字符串相加](https://leetcode.cn/problems/add-strings/)
给定两个字符串形式的非负整数 num1 和num2 ，计算它们的和并同样以字符串形式返回。

你不能使用任何內建的用于处理大整数的库（比如 BigInteger）， 也不能直接将输入的字符串转换为整数形式。



##### 示例 1：
```
输入：num1 = "11", num2 = "123"
输出："134"
```

##### 示例 2：
```
输入：num1 = "456", num2 = "77"
输出："533"
```

##### 示例 3：
```
输入：num1 = "0", num2 = "0"
输出："0"
```



##### 提示：
- 1 <= num1.length, num2.length <= 10<sup>4</sup>
- num1 和num2 都只包含数字 0-9
- num1 和num2 都不包含任何前导零

##### 题解：
```rust
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let n = num1.len();
        let m = num2.len();
        let mut ans = vec![];
        let mut i = n - 1;
        let mut j = m - 1;
        let mut c = 0;
        let n1 = num1.as_bytes();
        let n2 = num2.as_bytes();

        while i < n && j < m {
            let tmp = n1[i] as i32 + n2[j] as i32 + c - 96;
            ans.push(tmp % 10);
            c = tmp / 10;
            i -= 1;
            j -= 1;
        }

        while i < n {
            let tmp = n1[i] as i32 + c - 48;
            ans.push(tmp % 10);
            c = tmp / 10;
            i -= 1;
        }

        while j < m {
            let tmp = n2[j] as i32 + c - 48;
            ans.push(tmp % 10);
            c = tmp / 10;
            j -= 1;
        }

        if c != 0 {
            ans.push(c);
        }

        ans.into_iter().rev().map(|x| x.to_string()).collect::<String>()
    }
}
```
