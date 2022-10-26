### [18. 四数之和](https://leetcode.cn/problems/4sum/)

给你一个由 n 个整数组成的数组 nums ，和一个目标值 target 。请你找出并返回满足下述全部条件且 **不重复** 的四元组 [nums[a], nums[b], nums[c], nums[d]] （若两个四元组元素一一对应，则认为两个四元组重复）：

- 0 <= a, b, c, d < n
- a、b、c 和 d 互不相同
- nums[a] + nums[b] + nums[c] + nums[d] == target
你可以按 任意顺序 返回答案 。

##### 示例 1：
```
输入：nums = [1,0,-1,0,-2,2], target = 0
输出：[[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
```

##### 示例 2：
```
输入：nums = [2,2,2,2,2], target = 8
输出：[[2,2,2,2]]
```

##### 提示：
- 1 <= nums.length <= 200
- -10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>
- -10<sup>9</sup> <= target <= 10<sup>9</sup>

##### 题解：
```rust
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();

        if len < 4 {
            return vec![];
        }

        let mut ans = vec![];

        nums.sort();

        for i in 0..(len-3) {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }

            if (nums[i] as i64) + (nums[i+1] as i64) + (nums[i+2] as i64) + (nums[i+3] as i64) > (target as i64) {
                break;
            }

            if (nums[i] as i64) + (nums[len-3] as i64) + (nums[len-2] as i64) + (nums[len-1] as i64) < (target as i64) {
                continue;
            }

            for j in (i+1)..(len-2) {
                if j > i + 1 && nums[j] == nums[j-1] {
                    continue;
                }

                if (nums[i] as i64) + (nums[j] as i64) + (nums[j+1] as i64) + (nums[j+2] as i64) > (target as i64) {
                    break;
                }

                if (nums[i] as i64) + (nums[j] as i64) + (nums[len-2] as i64) + (nums[len-1] as i64) < (target as i64) {
                    continue;
                }

                let mut lo = j + 1;
                let mut hi = len - 1;

                while lo < hi && hi < len {
                    let sum = nums[i] + nums[j] + nums[lo] + nums[hi];
                    
                    if sum == target {
                        ans.push(vec![nums[i], nums[j], nums[lo], nums[hi]]);

                        while lo < hi && nums[lo] == nums[lo + 1] {
                            lo += 1;
                        }
                        lo += 1;

                        while lo < hi && hi < len && nums[hi] == nums[hi-1] {
                            hi -= 1;
                        }

                        hi -= 1;
                    } else if sum < target {
                        lo += 1;
                    } else {
                        hi -= 1;
                    }
                }
            }
        }

        ans
    }
}
```

`排序` `双指针`
