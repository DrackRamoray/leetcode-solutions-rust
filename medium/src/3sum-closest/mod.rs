struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut ans = 10_i32.pow(3);

        for i in 0..n {
            let mut j = i + 1;
            let mut k = n - 1;

            while j < k && k < n {
                let tmp = nums[i] + nums[j] + nums[k];

                if tmp == target {
                    return tmp;
                } else if tmp < target {
                    j += 1;
                } else {
                    k -= 1;
                }

                ans = if (ans-target).abs() < (tmp-target).abs() {
                    ans
                } else {
                    tmp
                };
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::three_sum_closest(vec![-1,2,1,-4],1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0,0,0],1), 0);
    }
}
