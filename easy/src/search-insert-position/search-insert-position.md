### [35. 搜索插入位置](https://leetcode.cn/problems/search-insert-position/)

给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。

请必须使用时间复杂度为 O(log n) 的算法。

##### 示例 1:
```
输入: nums = [1,3,5,6], target = 5
输出: 2
```

##### 示例 2:
```
输入: nums = [1,3,5,6], target = 2
输出: 1
```

##### 示例 3:
```
输入: nums = [1,3,5,6], target = 7
输出: 4
```

##### 提示:
- 1 <= nums.length <= 10<sup>4</sup>
- -10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>
- nums 为 无重复元素 的 升序 排列数组
- -10<sup>4</sup> <= target <= 10<sup>4</sup>

##### 题解：
```rust
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;
        let mut ans = n;

        while lo <= hi && hi < n {
            let mid = lo + (hi - lo) / 2;

            if nums[mid] < target {
                lo = mid + 1;
            } else {
                ans = mid;
                hi = mid - 1;
            }
        }

        ans as i32
    }
}
```

`二分查找`
