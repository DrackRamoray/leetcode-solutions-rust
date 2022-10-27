### [45. 跳跃游戏 II](https://leetcode.cn/problems/jump-game-ii/)

给你一个非负整数数组 nums ，你最初位于数组的第一个位置。

数组中的每个元素代表你在该位置可以跳跃的最大长度。

你的目标是使用最少的跳跃次数到达数组的最后一个位置。

假设你总是可以到达数组的最后一个位置。

##### 示例 1:
```
输入: nums = [2,3,1,1,4]
输出: 2
解释: 跳到最后一个位置的最小跳跃数是 2。
     从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后一个位置。
```

##### 示例 2:
```
输入: nums = [2,3,0,1,4]
输出: 2
```

##### 提示:
- 1 <= nums.length <= 10<sup>4</sup>
- 0 <= nums[i] <= 1000

##### 题解：
```rust
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0;n];

        for i in 0..n-1 {
            let num = nums[i] as usize;
            for j in 1..=num {
                if i + j >= n {
                    break;
                }

                dp[i+j] = if dp[i+j] == 0 {
                    dp[i] + 1
                } else {
                    dp[i+j].min(dp[i] + 1)
                };
            }
        }

        dp[n-1] as i32
    }
}
```

`动态规划`
