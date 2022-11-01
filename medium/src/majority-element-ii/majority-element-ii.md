### 229. 多数元素 II
给定一个大小为 n 的整数数组，找出其中所有出现超过 ⌊ n/3 ⌋ 次的元素。



##### 示例 1：
```
输入：nums = [3,2,3]
输出：[3]
```

##### 示例 2：
```
输入：nums = [1]
输出：[1]
```

##### 示例 3：
```
输入：nums = [1,2]
输出：[1,2]
```

##### 提示：
- 1 <= nums.length <= 5 * 10<sup>4</sup>
- -10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>


##### 进阶：
- 尝试设计时间复杂度为 O(n)、空间复杂度为 O(1)的算法解决此问题。

##### 题解：
```rust
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut major1 = nums[0];
        let mut count1 = 0_i32;

        let mut major2 = nums[0];
        let mut count2 = 0_i32;

        for &num in nums.iter() {
            if major1 == num {
                count1 += 1;
                continue;
            }

            if major2 == num {
                count2 += 1;
                continue;
            }

            if count1 == 0 {
                major1 = num;
                count1 += 1;
                continue;
            }

            if count2 == 0 {
                major2 = num;
                count2 += 1;
                continue;
            }

            count1 -= 1;
            count2 -= 1;
        }

        count1 = 0;
        count2 = 0;

        for &num in nums.iter() {
            if num == major1 {
                count1 += 1;
            }
            if num == major2 {
                count2 += 1;
            }
        }

        let n = (nums.len() / 3) as i32;
        let mut ans = Vec::new();

        if count1 > n {
            ans.push(major1);
        }

        if count2 > n && major1 != major2 {
            ans.push(major2);
        }

        ans
    }
}
```
