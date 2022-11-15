pub struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
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

#[test]
fn it_works() {
    assert_eq!(Solution::split_array(vec![7,2,5,10,8], 2), 18);
    assert_eq!(Solution::split_array(vec![1,2,3,4,5], 2), 9);
    assert_eq!(Solution::split_array(vec![1,4,4], 3), 4);
}
