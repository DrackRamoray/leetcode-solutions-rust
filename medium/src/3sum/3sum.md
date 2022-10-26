### [15. 三数之和](https://leetcode.cn/problems/3sum/)

给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请

你返回所有和为 0 且不重复的三元组。

**注意：** 答案中不可以包含重复的三元组。

##### 示例 1：
```
输入：nums = [-1,0,1,2,-1,-4]
输出：[[-1,-1,2],[-1,0,1]]
解释：
nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
注意，输出的顺序和三元组的顺序并不重要。
```

##### 示例 2：
```
输入：nums = [0,1,1]
输出：[]
解释：唯一可能的三元组和不为 0 。
```

##### 示例 2：
```
输入：nums = [0,0,0]
输出：[[0,0,0]]
解释：唯一可能的三元组和为 0 。
```

##### 提示：
- 3 <= nums.length <= 3000
- -10<sup>5</sup> <= nums[i] <= 10<sup>5</sup>

##### 题解：
```rust
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let n = nums.len();
        nums.sort();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i-1] { // 去重
                continue;
            }

            let mut j = n - 1;
            let t = -nums[i];

            for k in (i+1)..n {
                if k > i + 1 && nums[k] == nums[k-1] { // 去重
                    continue;
                }

                while k < j && j < n && nums[k] + nums[j] > t {
                    j -= 1;
                }

                if k == j {
                    break;
                }

                if nums[k] + nums[j] == t {
                    ans.push(vec![nums[i], nums[k], nums[j]]);
                }
            }
        }

        ans
    }
}
```

`排序` `双指针`
