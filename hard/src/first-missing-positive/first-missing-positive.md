### [41. 缺失的第一个正数](https://leetcode.cn/problems/first-missing-positive/)

给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。

请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。

##### 示例 1：
```
输入：nums = [1,2,0]
输出：3
```

##### 示例 2：
```
输入：nums = [3,4,-1,1]
输出：2
```

##### 示例 3：
```
输入：nums = [7,8,9,11,12]
输出：1
```

##### 提示：
- 1 <= nums.length <= 5 * 10<sup>5</sup>
- -2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1

##### 题解：
```rust
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let m = nums.len();

        nums.iter_mut().for_each(|x| if *x <= 0 { *x = m as i32 + 1 });

        for i in 0..m {
            let n = nums[i].abs() as usize;

            if n <= m {
                nums[n - 1] = -nums[n - 1].abs();
            }
        }

        for i in 0..m {
            if nums[i] > 0 {
                return i as i32 + 1;
            }
        }

        m as i32 + 1
    }
}
```

`原地哈希`
