### [324. 摆动排序 II](https://leetcode.cn/problems/wiggle-sort-ii/)
给你一个整数数组 nums，将它重新排列成 nums[0] < nums[1] > nums[2] < nums[3]... 的顺序。

你可以假设所有输入数组都可以得到满足题目要求的结果。



##### 示例 1：
```
输入：nums = [1,5,1,1,6,4]
输出：[1,6,1,5,1,4]
解释：[1,4,1,5,1,6] 同样是符合题目要求的结果，可以被判题程序接受。
```

##### 示例 2：
```
输入：nums = [1,3,2,2,3,1]
输出：[2,3,1,3,1,2]
```

##### 提示：
- 1 <= nums.length <= 5 * 10<sup>4</sup>
- 0 <= nums[i] <= 5000
- 题目数据保证，对于给定的输入 nums ，总能产生满足题目要求的结果


##### 进阶：
- 你能用 O(n) 时间复杂度和 / 或原地 O(1) 额外空间来实现吗？

##### 题解：
```rust
impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut nums1 = nums.clone();

        nums1.sort();
        
        let mut i = (n - 1) / 2;
        let mut j = n - 1;

        for k in 0..n {
            if k % 2 == 0 {
                nums[k] = nums1[i];
                i -= 1;
            } else {
                nums[k] = nums1[j];
                j -= 1;
            }
        }
    }
}
```
