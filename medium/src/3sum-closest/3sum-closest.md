### [16. 最接近的三数之和](https://leetcode.cn/problems/3sum-closest/)

给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。

返回这三个数的和。

假定每组输入只存在恰好一个解。

##### 示例 1：
```
输入：nums = [-1,2,1,-4], target = 1
输出：2
解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。
```

##### 示例 2：
```
输入：nums = [0,0,0], target = 1
输出：0
```

##### 提示：
- 3 <= nums.length <= 1000
- -1000 <= nums[i] <= 1000
- -10<sup>4</sup> <= target <= 10<sup>4</sup>

##### 题解：
```rust
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut ans = 10_i32.pow(3);

        for i in 0..n {
            let mut j = i + 1;
            let mut k = n - 1;

            while j < k && k < n {
                let tmp = nums[i] + nums[j] + nums[k];

                if tmp == target {
                    return tmp;
                } else if tmp < target {
                    j += 1;
                } else {
                    k -= 1;
                }

                ans = if (ans-target).abs() < (tmp-target).abs() {
                    ans
                } else {
                    tmp
                };
            }
        }

        ans
    }
}
```

`排序` `双指针`
