### [75. 颜色分类](https://leetcode.cn/problems/sort-colors/)

给定一个包含红色、白色和蓝色、共 n 个元素的数组 nums ，原地对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。

我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。

必须在不使用库的sort函数的情况下解决这个问题。


##### 示例 1：
```
输入：nums = [2,0,2,1,1,0]
输出：[0,0,1,1,2,2]
```

##### 示例 2：
```
输入：nums = [2,0,1]
输出：[0,1,2]
```

##### 提示：
- n == nums.length
- 1 <= n <= 300
- nums[i] 为 0、1 或 2

##### 进阶：
- 你可以不使用代码库中的排序函数来解决这道题吗？
- 你能想出一个仅使用常数空间的一趟扫描算法吗？

##### 题解：
```rust
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;
        let mut p = 0;

        while p <= hi && hi < n {
            while hi < n && nums[hi] == 2 {
                hi -= 1;
            }
            if nums[p] == 0 {
                nums.swap(p, lo);
                lo += 1;
                p += 1;
            } else if nums[p] == 2 && hi < n {
                nums.swap(p, hi);
                hi -= 1;
            } else {
                p += 1;
            }
        }
    }
}
```

`双指针`
