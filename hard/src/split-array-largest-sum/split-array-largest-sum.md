### [410. 分割数组的最大值](https://leetcode.cn/problems/split-array-largest-sum/)
给定一个非负整数数组 nums 和一个整数 m ，你需要将这个数组分成 m 个非空的连续子数组。

设计一个算法使得这 m 个子数组各自和的最大值最小。



##### 示例 1：
```
输入：nums = [7,2,5,10,8], m = 2
输出：18
解释：
一共有四种方法将 nums 分割为 2 个子数组。
其中最好的方式是将其分为 [7,2,5] 和 [10,8] 。
因为此时这两个子数组各自的和的最大值为18，在所有情况中最小。
```

##### 示例 2：
```
输入：nums = [1,2,3,4,5], m = 2
输出：9
```

##### 示例 3：
```
输入：nums = [1,4,4], m = 3
输出：4
```

##### 提示：
- 1 <= nums.length <= 1000
- 0 <= nums[i] <= 10<sup>6</sup>
- 1 <= m <= min(50, nums.length)

##### 题解：
```rust
impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let n = nums.len();
        let mut lo = *nums.iter().max().unwrap();
        let mut hi = nums.iter().sum::<i32>();

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if Self::get_cnt(&nums, mid) <= m {
                hi = mid;
            } else {
                lo = mid + 1; 
            }
        }

        lo
    }

    fn get_cnt(nums: &Vec<i32>, target: i32) -> i32 {
        let mut sum = 0;
        let mut cnt = 1;

        for &num in nums.iter() {
            if sum + num > target {
                cnt += 1;
                sum = num;
            } else {
                sum += num;
            }
        }

        cnt
    }
}
```
