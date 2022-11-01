### [238. 除自身以外数组的乘积](https://leetcode.cn/problems/product-of-array-except-self/)
给你一个整数数组 nums，返回 数组 answer ，其中 answer[i] 等于 nums 中除 nums[i] 之外其余各元素的乘积 。

题目数据 保证 数组 nums之中任意元素的全部前缀元素和后缀的乘积都在  32 位 整数范围内。

请不要使用除法，且在 O(n) 时间复杂度内完成此题。



##### 示例 1:
```
输入: nums = [1,2,3,4]
输出: [24,12,8,6]
```

##### 示例 2:
```
输入: nums = [-1,1,0,-3,3]
输出: [0,0,9,0,0]
```

##### 提示：
- 2 <= nums.length <= 10<sup>5</sup>
- -30 <= nums[i] <= 30
- 保证 数组 nums之中任意元素的全部前缀元素和后缀的乘积都在  32 位 整数范围内


##### 进阶：
- 你可以在 O(1) 的额外空间复杂度内完成这个题目吗？（ 出于对空间复杂度分析的目的，输出数组不被视为额外空间。）

##### 题解：
```rust
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut p = 1;
        let mut ans = vec![1;n];

        for i in 0..n {
            ans[i] = p;
            p = p * nums[i];
        }

        let mut s = 1;

        for i in (1..n).rev() {
            s = s * nums[i];
            ans[i-1] = ans[i-1] * s;
        }

        ans
    }
}
```
