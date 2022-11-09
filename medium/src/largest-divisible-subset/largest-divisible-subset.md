### [368. 最大整除子集](https://leetcode.cn/problems/largest-divisible-subset/)
给你一个由 无重复 正整数组成的集合 nums ，请你找出并返回其中最大的整除子集 answer ，子集中每一元素对 (answer[i], answer[j]) 都应当满足：
- answer[i] % answer[j] == 0 ，或
- answer[j] % answer[i] == 0

如果存在多个有效解子集，返回其中任何一个均可。



##### 示例 1：
```
输入：nums = [1,2,3]
输出：[1,2]
解释：[1,3] 也会被视为正确答案。
```

##### 示例 2：
```
输入：nums = [1,2,4,8]
输出：[1,2,4,8]
```

##### 提示：
- 1 <= nums.length <= 1000
- 1 <= nums[i] <= 2 * 10<sup>9</sup>
- nums 中的所有整数 互不相同

##### 题解：
```rust
impl Solution {
     pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
          nums.sort(); // 排序
          let n = nums.len();

          if n == 0 {
               return Vec::new();
          }

          let mut dp = vec![vec![nums[0]]]; // nums[i]的整除子集
          let mut ans = dp[0].clone();

          for i in 1..n {
               dp.push(vec![nums[i]]);

               for j in 0..i {
                    if nums[i] % dp[j][dp[j].len() - 1] == 0 { // 最后一个能整除
                         if dp[j].len() >= dp[i].len() - 1 {
                              dp[i] = dp[j].clone();
                              dp[i].push(nums[i]);
                         }
                    }
               }

               if ans.len() < dp[i].len() {
                    ans = dp[i].clone();
               }
          }

          ans
     }
}
```
