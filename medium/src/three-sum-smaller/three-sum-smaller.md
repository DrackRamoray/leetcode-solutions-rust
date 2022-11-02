### [259. 较小的三数之和](https://leetcode.cn/problems/3sum-smaller/)
给定一个长度为 n 的整数数组和一个目标值 target ，寻找能够使条件 nums[i] + nums[j] + nums[k] < target 成立的三元组  i, j, k 个数（0 <= i < j < k < n）。



##### 示例 1：
```
输入: nums = [-2,0,1,3], target = 2
输出: 2
解释: 因为一共有两个三元组满足累加和小于 2:
[-2,0,1]
[-2,0,3]
```

##### 示例 2：
```
输入: nums = [], target = 0
输出: 0
```

##### 示例 3：
```
输入: nums = [0], target = 0
输出: 0
```

##### 提示:
- n == nums.length
- 0 <= n <= 3500
- -100 <= nums[i] <= 100
- -100 <= target <= 100

##### 题解：
```rust
impl Solution {
    pub fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            return 0;
        }

        nums.sort();

        let mut sum = 0;
        let to = nums.len() - 2;

        for i in 0..to {
            sum += Self::two_sum(&nums, i + 1, target - nums[i])
        }

        sum
    }

    fn two_sum(nums: &Vec<i32>, start: usize, target: i32) -> i32 {
        let mut sum = 0;
        let mut left = start;
        let mut right = nums.len() - 1;

        while left < right {
            if nums[left] + nums[right] < target {
                sum += right - left;
                left += 1;
            } else {
                right -= 1;
            }
        }

        sum as i32
    }
}
```
