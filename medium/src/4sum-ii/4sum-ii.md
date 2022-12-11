### [454. 四数相加 II](https://leetcode.cn/problems/4sum-ii/)
给你四个整数数组 nums1、nums2、nums3 和 nums4 ，数组长度都是 n ，请你计算有多少个元组 (i, j, k, l) 能满足：

- 0 <= i, j, k, l < n
- nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0


##### 示例 1：
```
输入：nums1 = [1,2], nums2 = [-2,-1], nums3 = [-1,2], nums4 = [0,2]
输出：2
解释：
两个元组如下：
1. (0, 0, 0, 1) -> nums1[0] + nums2[0] + nums3[0] + nums4[1] = 1 + (-2) + (-1) + 2 = 0
2. (1, 1, 0, 0) -> nums1[1] + nums2[1] + nums3[0] + nums4[0] = 2 + (-1) + (-1) + 0 = 0
```

##### 示例 2：
```
输入：nums1 = [0], nums2 = [0], nums3 = [0], nums4 = [0]
输出：1
```

##### 提示：
- n == nums1.length
- n == nums2.length
- n == nums3.length
- n == nums4.length
- 1 <= n <= 200
- -2<sup>28</sup> <= nums1[i], nums2[i], nums3[i], nums4[i] <= 2<sup>28</sup>

##### 题解：
```rust
impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut mp = std::collections::HashMap::new();

        for &num3 in nums3.iter() {
            for &num4 in nums4.iter() {
                *mp.entry(num3 + num4).or_insert(0) += 1;
            }
        }

        let mut ans = 0;

        for &num1 in nums1.iter() {
            for &num2 in nums2.iter() {
                if let Some(num) = mp.get(&(-num1-num2)) {
                    ans += num;
                }
            }
        }

        ans
    }
}
```
