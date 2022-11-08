### [360. 有序转化数组](https://leetcode.cn/problems/sort-transformed-array/)
给你一个已经 排好序 的整数数组 nums 和整数 a 、 b 、 c 。对于数组中的每一个元素 nums[i] ，计算函数值 f(x) = ax2 + bx + c ，请 按升序返回数组 。



##### 示例 1：
```
输入: nums = [-4,-2,2,4], a = 1, b = 3, c = 5
输出: [3,9,15,33]
```

##### 示例 2：
```
输入: nums = [-4,-2,2,4], a = -1, b = 3, c = 5
输出: [-23,-5,1,7]
```

##### 提示：
- 1 <= nums.length <= 200
- -100 <= nums[i], a, b, c <= 100
- nums 按照 升序排列


##### 进阶：
- 你可以在时间复杂度为 O(n) 的情况下解决这个问题吗？

##### 题解：
```rust
impl Solution {
    pub fn sort_transformed_array(nums: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32> {
        let n = nums.len();

        if n == 0 {
            return vec![];
        }

        let fx = |x: i32| a * x * x + b * x + c;

        let mut i = 0;
        let mut j = n - 1;
        let mut index = if a >= 0 {
            n - 1
        } else {
            0
        };
        let mut ans = vec![0;n];

        while i <= j {
            let t1 = fx(nums[i]);
            let t2 = fx(nums[j]);

            if a >= 0 {
                ans[index] = if t1 >= t2 {
                    i += 1;
                    t1
                } else {
                    j -= 1;
                    t2
                };

                index -= 1;
            } else {
                ans[index] = if t1 >= t2 {
                    j -= 1;
                    t2
                } else {
                    i += 1;
                    t1
                };

                index += 1;
            }
        }

        ans
    }
}
```
