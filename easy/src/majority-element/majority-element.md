### [169. 多数元素](https://leetcode.cn/problems/majority-element/)
给定一个大小为 n 的数组 nums ，返回其中的多数元素。多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。

你可以假设数组是非空的，并且给定的数组总是存在多数元素。



##### 示例 1：
```
输入：nums = [3,2,3]
输出：3
```

##### 示例 2：
```
输入：nums = [2,2,1,1,1,2,2]
输出：2
```

##### 提示：
- n == nums.length
- 1 <= n <= 5 * 10<sup>4</sup>
- -10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>


##### 进阶：
- 尝试设计时间复杂度为 O(n)、空间复杂度为 O(1) 的算法解决此问题。

##### 题解：
```rust
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut candidate = None;

        for &num in nums.iter() {
            if cnt == 0 {
                candidate = Some(num);
            }
            if let Some(v) = candidate {
                if v == num {
                    cnt += 1;
                } else {
                    cnt -= 1;
                }
            } else {
                cnt -= 1;
            }
        }

        candidate.unwrap()
    }
}
```
