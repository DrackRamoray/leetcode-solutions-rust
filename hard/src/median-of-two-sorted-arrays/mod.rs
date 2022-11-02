pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let n = nums1.len();
        let m = nums2.len();

        let mut left = 0;
        let mut right = n;

        let mut max_left = 0;
        let mut min_right = 0;

        while left <= right && right <= n {
            let i = (left + right) / 2;
            let j = (n + m + 1) / 2 - i;

            let num1 = if i == 0 { i32::MIN } else { nums1[i-1] };
            let num2 = if j == 0 { i32::MIN } else { nums2[j-1] };
            let num3 = if i == n { i32::MAX } else { nums1[i] };
            let num4 = if j == m { i32::MAX } else { nums2[j] };

            if num1 <= num4 {
                max_left = num1.max(num2);
                min_right = num3.min(num4);
                left = i + 1;
            } else {
                right = i - 1;
            }
        }

        if (n + m) % 2 == 0 {
            (max_left as f64 + min_right as f64) / 2.0
        } else {
            max_left as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,3], vec![2]), 2.00000);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.50000);
    }
}
