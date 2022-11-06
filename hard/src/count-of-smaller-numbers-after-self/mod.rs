pub struct Solution;

impl Solution {
    pub fn count_smaller(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut tmp = vec![0;n];
        let mut index = vec![0;n];
        let mut tmp_index = vec![0;n];
        let mut ans = vec![0;n];

        for i in 0..n {
            index[i] = i;
        }

        Self::merge_sort(&mut tmp, &mut index, &mut tmp_index, &mut ans, &mut nums, 0, n - 1);

        ans
    }

    fn merge_sort(tmp: &mut Vec<i32>, index: &mut Vec<usize>, tmp_index: &mut Vec<usize>, ans: &mut Vec<i32>, nums: &mut Vec<i32>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mid = left + (right - left) / 2;

        Self::merge_sort(tmp, index, tmp_index, ans, nums, left, mid);
        Self::merge_sort(tmp, index, tmp_index, ans, nums, mid + 1, right);

        Self::merge(tmp, index, tmp_index, ans, nums, left, mid, right);
    }

    fn merge(tmp: &mut Vec<i32>, index: &mut Vec<usize>, tmp_index: &mut Vec<usize>, ans: &mut Vec<i32>, nums: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
        let mut i = left;
        let mut j = mid + 1;
        let mut p = left;

        while i <= mid && j <= right {
            if nums[i] <= nums[j] {
                tmp[p] = nums[i];
                tmp_index[p] = index[i];
                ans[index[i]] += (j - mid - 1) as i32;
                i += 1;
                p += 1;
            } else {
                tmp[p] = nums[j];
                tmp_index[p] = index[j];
                j += 1;
                p += 1;
            }
        }

        while i <= mid {
            tmp[p] = nums[i];
            tmp_index[p] = index[i];
            ans[index[i]] += (j - mid - 1) as i32;
            i += 1;
            p += 1;
        }

        while j <= right {
            tmp[p] = nums[j];
            tmp_index[p] = index[j];
            j += 1;
            p += 1;
        }

        for k in left..=right {
            index[k] = tmp_index[k];
            nums[k] = tmp[k];
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::count_smaller(vec![5,2,6,1]), vec![2,1,1,0]);
    assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
    assert_eq!(Solution::count_smaller(vec![-1,-1]), vec![0,0]);
}
