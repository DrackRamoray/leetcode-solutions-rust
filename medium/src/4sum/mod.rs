pub struct Solution;

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

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::four_sum(vec![1,0,-1,0,-2,2], 0), vec![vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]]);
        assert_eq!(Solution::four_sum(vec![2,2,2,2,2], 8), vec![vec![2,2,2,2]]);
    }
}
