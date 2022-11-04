pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        if len < 1 {
            return 0
        }

        let mut dp = vec![i32::MAX;len+1];
        let mut count = 1;
        dp[count] = nums[0];

        for i in 1..len {
            if nums[i] > dp[count] {
                count = count + 1;
                dp[count] = nums[i];
            } else {
                let mut pos = 0;
                let mut l = 1;
                let mut r = len;

                while l <= r {
                    let m = (l + r) >> 1;

                    if nums[i] > dp[m] {
                        pos = m;
                        l = m + 1;
                    } else {
                        r = m - 1;
                    }
                }

                dp[pos + 1] = nums[i];
            }
        }

        count as i32
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::length_of_lis(vec![10,9,2,5,3,7,101,18]), 4);
    assert_eq!(Solution::length_of_lis(vec![0,1,0,3,2,3]), 4);
    assert_eq!(Solution::length_of_lis(vec![7,7,7,7,7,7,7]), 1);
}
