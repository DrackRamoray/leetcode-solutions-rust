### [325. 和等于 k 的最长子数组长度](https://leetcode.cn/problems/maximum-size-subarray-sum-equals-k/)
给定一个数组 nums 和一个目标值 k，找到和等于 k 的最长连续子数组长度。如果不存在任意一个符合要求的子数组，则返回 0。



##### 示例 1:
```
输入: nums = [1,-1,5,-2,3], k = 3
输出: 4
解释: 子数组 [1, -1, 5, -2] 和等于 3，且长度最长。
```

##### 示例 2:
```
输入: nums = [-2,-1,2,1], k = 1
输出: 2
解释: 子数组 [-1, 2] 和等于 1，且长度最长。
```

##### 提示：
- 1 <= nums.length <= 2 * 10<sup>5</sup>
- -10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>
- -10<sup>9</sup> <= k <= 10<sup>9</sup>

##### 题解：
```rust
impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut m = std::collections::HashMap::new();
        m.insert(0, -1);
        let mut ans = 0;
        let mut pre_sum = 0;

        for i in 0..len {
            pre_sum += nums[i];

            if !m.contains_key(&pre_sum) {
                m.insert(pre_sum, i as i32);
            }

            if let Some(&v) = m.get(&(pre_sum - k)) {
                ans = ans.max(i as i32 - v);
            } 
        }

        ans
    }
}
```
