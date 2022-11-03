### [283. 移动零](https://leetcode.cn/problems/move-zeroes/)
给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。

请注意 ，必须在不复制数组的情况下原地对数组进行操作。



##### 示例 1:
```
输入: nums = [0,1,0,3,12]
输出: [1,3,12,0,0]
```

示例 2:
```
输入: nums = [0]
输出: [0]
```

#####提示:
- 1 <= nums.length <= 10<sup>4</sup>
- -2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1


#####进阶：
- 你能尽量减少完成的操作次数吗？

##### 题解：
```rust
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut lo = 0;
        let mut hi = 0;

        while hi < nums.len() {
            if nums[hi] != 0 {
                nums.swap(lo, hi);
                lo += 1;
            }

            hi += 1;
        }
    }
}
```
