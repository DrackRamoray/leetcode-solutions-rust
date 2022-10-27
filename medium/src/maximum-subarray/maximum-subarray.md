### [53. 最大子数组和](https://leetcode.cn/problems/maximum-subarray/)

给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

子数组 是数组中的一个连续部分。

##### 示例 1：
```
输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
输出：6
解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
```

##### 示例 2：
```
输入：nums = [1]
输出：1
```

##### 示例 3：
```
输入：nums = [5,4,-1,7,8]
输出：23
```

##### 提示：
- 1 <= nums.length <= 10<sup>5</sup>
- -10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>

##### 题解：
```rust
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let mut prev_sum = 0;

        for num in nums.into_iter() {
            prev_sum = num.max(prev_sum + num);
            ans = ans.max(prev_sum);
        }

        ans
    }
}
```

`动态规划` `子数组`
