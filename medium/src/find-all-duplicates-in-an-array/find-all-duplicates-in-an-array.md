### [442. 数组中重复的数据](https://leetcode.cn/problems/find-all-duplicates-in-an-array/)
给你一个长度为 n 的整数数组 nums ，其中 nums 的所有整数都在范围 [1, n] 内，且每个整数出现 一次 或 两次 。请你找出所有出现 两次 的整数，并以数组形式返回。

你必须设计并实现一个时间复杂度为 O(n) 且仅使用常量额外空间的算法解决此问题。



##### 示例 1：
```
输入：nums = [4,3,2,7,8,2,3,1]
输出：[2,3]
```

##### 示例 2：
```
输入：nums = [1,1,2]
输出：[1]
```

##### 示例 3：
```
输入：nums = [1]
输出：[]
```

##### 提示：
- n == nums.length
- 1 <= n <= 10<sup>5</sup>
- 1 <= nums[i] <= n
- nums 中的每个元素出现 一次 或 两次

##### 题解：
```rust
impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        for i in 0..nums.len() {
            let index = nums[i].abs() as usize - 1;

            if nums[index] > 0 {
                nums[index] = -nums[index];
            } else {
                ans.push(index as i32 + 1)
            }
        }

        ans
    }
}
```
