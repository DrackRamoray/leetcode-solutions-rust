pub struct Solution;

impl Solution {
    pub fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            return 0;
        }

        nums.sort();

        let mut sum = 0;
        let to = nums.len() - 2;

        for i in 0..to {
            sum += Self::two_sum(&nums, i + 1, target - nums[i])
        }

        sum
    }

    fn two_sum(nums: &Vec<i32>, start: usize, target: i32) -> i32 {
        let mut sum = 0;
        let mut left = start;
        let mut right = nums.len() - 1;

        while left < right {
            if nums[left] + nums[right] < target {
                sum += right - left;
                left += 1;
            } else {
                right -= 1;
            }
        }

        sum as i32
    }
}

#[test]
fn it_works() {
    let nums = vec![-2, 0, 1, 3];
    let target = 2;
    let ans = 2;
    assert_eq!(Solution::three_sum_smaller(nums, target), ans);
}
