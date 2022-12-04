### [440. 字典序的第K小数字](https://leetcode.cn/problems/k-th-smallest-in-lexicographical-order/)
给定整数 n 和 k，返回  [1, n] 中字典序第 k 小的数字。



##### 示例 1:
```
输入: n = 13, k = 2
输出: 10
解释: 字典序的排列是 [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]，所以第二小的数字是 10。
```

##### 示例 2:
```
输入: n = 1, k = 1
输出: 1
```

##### 提示:
- 1 <= k <= n <= 10<sup>9</sup>

##### 题解：
```rust
impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut ans = 1;
        let mut k = k - 1;

        while k > 0 {
            let steps = Self::get_steps(ans, n as u64);

            if steps <= k {
                k -= steps;
                ans += 1;
            } else {
                ans *= 10;
                k -= 1;
            }
        }

        ans
    }

    fn get_steps(cur: i32, n: u64) -> i32 {
        let mut cnt = 0;
        let mut prev = cur as u64;
        let mut next = cur as u64;

        while prev <= n {
            cnt += next.min(n) - prev + 1;
            prev *= 10;
            next = next * 10 + 9;
        }

        cnt as i32
    }
}
```
