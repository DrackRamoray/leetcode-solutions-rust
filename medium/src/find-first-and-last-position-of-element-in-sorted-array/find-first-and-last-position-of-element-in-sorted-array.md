### [34. 在排序数组中查找元素的第一个和最后一个位置](https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/)

给你一个按照非递减顺序排列的整数数组 nums，和一个目标值 target。请你找出给定目标值在数组中的开始位置和结束位置。

如果数组中不存在目标值 target，返回 [-1, -1]。

你必须设计并实现时间复杂度为 O(log n) 的算法解决此问题。

##### 示例 1：
```
输入：nums = [5,7,7,8,8,10], target = 8
输出：[3,4]
```

##### 示例 2：
```
输入：nums = [5,7,7,8,8,10], target = 6
输出：[-1,-1]
```

##### 示例 3：
```
输入：nums = [], target = 0
输出：[-1,-1]
```

##### 提示：
- 0 <= nums.length <= 10<sup>5</sup>
- -10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>
- nums 是一个非递减数组
- -10<sup>9</sup> <= target <= 10<sup>9</sup>


##### 题解：
```rust
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let left = Self::left_bound(&nums, target);

        if left == -1 {
            return vec![-1, -1];
        }

        let right = Self::right_bound(&nums, target);

        vec![left, right]
    }

    fn left_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;
        let mut ans = n;

        while lo <= hi && hi < n {
            let mid = lo + (hi - lo) / 2;

            if nums[mid] >= target {
                hi = mid - 1;
                ans = mid;
            } else {
                lo = mid + 1; 
            }
        }

        if ans == n || nums[ans] != target {
            -1
        } else {
            ans as i32
        }
    }

    fn right_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;
        let mut ans = n;

        while lo <= hi && hi < n {
            let mid = lo + (hi - lo) / 2;

            if nums[mid] <= target {
                lo = mid + 1;
                ans = mid;
            } else {
                hi = mid - 1; 
            }
        }

        if ans == n || nums[ans] != target {
            -1
        } else {
            ans as i32
        }
    }
}
```

`二分查找`
