### [209. 长度最小的子数组](https://leetcode.cn/problems/minimum-size-subarray-sum/)
给定一个含有 n 个正整数的数组和一个正整数 target 。

找出该数组中满足其和 ≥ target 的长度最小的 连续子数组 [numsl, numsl+1, ..., numsr-1, numsr] ，并返回其长度。如果不存在符合条件的子数组，返回 0 。



##### 示例 1：
```
输入：target = 7, nums = [2,3,1,2,4,3]
输出：2
解释：子数组 [4,3] 是该条件下的长度最小的子数组。
```

##### 示例 2：
```
输入：target = 4, nums = [1,4,4]
输出：1
```

##### 示例 3：
```
输入：target = 11, nums = [1,1,1,1,1,1,1,1]
输出：0
```

##### 提示：
- 1 <= target <= 10<sup>9</sup>
- 1 <= nums.length <= 10<sup>5</sup>
- 1 <= nums[i] <= 10<sup>5</sup>


##### 进阶：
- 如果你已经实现 O(n) 时间复杂度的解法, 请尝试设计一个 O(n log(n)) 时间复杂度的解法。

##### 题解：
```rust
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        if n == 0 {
            return 0;
        }

        let mut ans = n + 1;
        let mut left = 0;
        let mut sum = 0;

        for right in 0..n {
            sum += nums[right];

            while sum >= target && left <= right {
                ans = ans.min(right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }

        if ans == n + 1 {
            return 0;
        }

        ans as i32
    }
}
```
