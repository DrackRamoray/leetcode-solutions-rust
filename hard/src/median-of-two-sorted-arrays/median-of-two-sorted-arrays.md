### [4. 寻找两个正序数组的中位数](https://leetcode.cn/problems/median-of-two-sorted-arrays/)

给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。

算法的时间复杂度应该为 O(log (m+n)) 。

##### 示例 1：
```
输入：nums1 = [1,3], nums2 = [2]
输出：2.00000
解释：合并数组 = [1,2,3] ，中位数 2
```

##### 示例 2：
```
输入：nums1 = [1,2], nums2 = [3,4]
输出：2.50000
解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
```

##### 提示：
- nums1.length == m
- nums2.length == n
- 0 <= m <= 1000
- 0 <= n <= 1000
- 1 <= m + n <= 2000
- -10<sup>6</sup> <= nums1[i], nums2[i] <= 10<sup>6</sup>

##### 题解：
```rust
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let n = nums1.len();
        let m = nums2.len();

        let mut left = 0;
        let mut right = n;

        let mut max_left = 0;
        let mut min_right = 0;

        while left <= right && right <= n {
            let i = (left + right) / 2;
            let j = (n + m + 1) / 2 - i;

            let num1 = if i == 0 { i32::MIN } else { nums1[i-1] };
            let num2 = if j == 0 { i32::MIN } else { nums2[j-1] };
            let num3 = if i == n { i32::MAX } else { nums1[i] };
            let num4 = if j == m { i32::MAX } else { nums2[j] };
            
            if num1 <= num4 {
                max_left = num1.max(num2);
                min_right = num3.min(num4);
                left = i + 1;
            } else {
                right = i - 1;
            }
        }

        if (n + m) % 2 == 0 {
            (max_left as f64 + min_right as f64) / 2.0
        } else {
            max_left as f64
        }
    }
}
```

`二分查找`
