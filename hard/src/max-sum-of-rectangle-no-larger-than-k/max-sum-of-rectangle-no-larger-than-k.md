### [363. 矩形区域不超过 K 的最大数值和](https://leetcode.cn/problems/max-sum-of-rectangle-no-larger-than-k/)
给你一个 m x n 的矩阵 matrix 和一个整数 k ，找出并返回矩阵内部矩形区域的不超过 k 的最大数值和。

题目数据保证总会存在一个数值和不超过 k 的矩形区域。



##### 示例 1：
![img.png](img.png)
```
输入：matrix = [[1,0,1],[0,-2,3]], k = 2
输出：2
解释：蓝色边框圈出来的矩形区域 [[0, 1], [-2, 3]] 的数值和是 2，且 2 是不超过 k 的最大数字（k = 2）。
```

##### 示例 2：
```
输入：matrix = [[2,2,-1]], k = 3
输出：3
```

##### 提示：
- m == matrix.length
- n == matrix[i].length
- 1 <= m, n <= 100
- -100 <= matrix[i][j] <= 100
- -10<sup>5</sup> <= k <= 10<sup>5</sup>


##### 进阶：
- 如果行数远大于列数，该如何设计解决方案？

##### 题解：
```rust
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = i32::MIN;
        let n = matrix.len();
        let m = matrix[0].len();

        for i in 1..=n {
            for j in 1..=m {
                let mut dp = vec![vec![0;m+1];n+1];
                dp[i][j] = matrix[i-1][j-1];
                for n in i..=n {
                    for m in j..=m {
                        dp[n][m] = dp[n-1][m] + dp[n][m-1] - dp[n-1][m-1] + matrix[n-1][m-1];

                        if dp[n][m] <= k && dp[n][m] > ans {
                            ans = dp[n][m];
                        }
                    }
                }
            }
        }

        ans
    }
}
```
