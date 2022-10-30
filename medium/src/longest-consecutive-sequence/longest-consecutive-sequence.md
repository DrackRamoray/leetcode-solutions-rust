### [128. 最长连续序列](https://leetcode.cn/problems/longest-consecutive-sequence/)

给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。

请你设计并实现时间复杂度为 O(n) 的算法解决此问题。



##### 示例 1：
```
输入：nums = [100,4,200,1,3,2]
输出：4
解释：最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。
```

##### 示例 2：
```
输入：nums = [0,3,7,2,5,8,4,6,0,1]
输出：9
```

##### 提示：
- 0 <= nums.length <= 10<sup>5</sup>
- -10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>

##### 题解：
```rust
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut num_set = std::collections::HashSet::new();

        for &num in nums.iter() {
            num_set.insert(num);
        }

        let mut ans = 0;

        for &num in num_set.iter() {
            if !num_set.contains(&(num - 1)) {
                let mut cur = num;
                let mut cnt = 1;

                while num_set.contains(&(cur + 1)) {
                    cur += 1;
                    cnt += 1;
                }

                ans = ans.max(cnt);
            }
        }

        ans
    }
}
```

`哈希`
