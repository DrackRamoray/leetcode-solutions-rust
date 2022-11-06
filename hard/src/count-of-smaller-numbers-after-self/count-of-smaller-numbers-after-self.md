### [315. 计算右侧小于当前元素的个数](https://leetcode.cn/problems/count-of-smaller-numbers-after-self/)
给你一个整数数组 nums ，按要求返回一个新数组 counts 。数组 counts 有该性质： counts[i] 的值是  nums[i] 右侧小于 nums[i] 的元素的数量。



##### 示例 1：
```
输入：nums = [5,2,6,1]
输出：[2,1,1,0]
解释：
5 的右侧有 2 个更小的元素 (2 和 1)
2 的右侧仅有 1 个更小的元素 (1)
6 的右侧有 1 个更小的元素 (1)
1 的右侧有 0 个更小的元素
```

##### 示例 2：
```
输入：nums = [-1]
输出：[0]
```

##### 示例 3：
```
输入：nums = [-1,-1]
输出：[0,0]
```

##### 提示：
- 1 <= nums.length <= 10<sup>5</sup>
- -10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>

##### 题解：
```rust
impl Solution {
    pub fn count_smaller(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut tmp = vec![0;n];
        let mut index = vec![0;n];
        let mut tmpIndex = vec![0;n];
        let mut ans = vec![0;n];

        for i in 0..n {
            index[i] = i;
        }

        Self::merge_sort(&mut tmp, &mut index, &mut tmpIndex, &mut ans, &mut nums, 0, n - 1);

        ans
    }

    fn merge_sort(tmp: &mut Vec<i32>, index: &mut Vec<usize>, tmpIndex: &mut Vec<usize>, ans: &mut Vec<i32>, nums: &mut Vec<i32>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mid = left + (right - left) / 2;

        Self::merge_sort(tmp, index, tmpIndex, ans, nums, left, mid);
        Self::merge_sort(tmp, index, tmpIndex, ans, nums, mid + 1, right);

        Self::merge(tmp, index, tmpIndex, ans, nums, left, mid, right);
    }

    fn merge(tmp: &mut Vec<i32>, index: &mut Vec<usize>, tmpIndex: &mut Vec<usize>, ans: &mut Vec<i32>, nums: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
        let mut i = left;
        let mut j = mid + 1;
        let mut p = left;

        while i <= mid && j <= right {
            if nums[i] <= nums[j] {
                tmp[p] = nums[i];
                tmpIndex[p] = index[i];
                ans[index[i]] += (j - mid - 1) as i32;
                i += 1;
                p += 1;
            } else {
                tmp[p] = nums[j];
                tmpIndex[p] = index[j];
                j += 1;
                p += 1;
            }
        }

        while i <= mid {
            tmp[p] = nums[i];
            tmpIndex[p] = index[i];
            ans[index[i]] += (j - mid - 1) as i32;
            i += 1;
            p += 1;
        }

        while j <= right {
            tmp[p] = nums[j];
            tmpIndex[p] = index[j];
            j += 1;
            p += 1;
        }

        for k in left..=right {
            index[k] = tmpIndex[k];
            nums[k] = tmp[k];
        }
    }
}
```
