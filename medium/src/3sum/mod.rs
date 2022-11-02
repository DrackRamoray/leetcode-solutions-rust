struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let n = nums.len();
        nums.sort();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }

            let mut j = n - 1;
            let t = -nums[i];

            for k in (i+1)..n {
                if k > i + 1 && nums[k] == nums[k-1] {
                    continue;
                }

                while k < j && j < n && nums[k] + nums[j] > t {
                    j -= 1;
                }

                if k == j {
                    break;
                }

                if nums[k] + nums[j] == t {
                    ans.push(vec![nums[i], nums[k], nums[j]]);
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
        assert_eq!(Solution::three_sum(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2],vec![-1,0,1]]);
        assert_eq!(Solution::three_sum(vec![0,1,1]), vec![] as Vec<Vec<i32>>);
        assert_eq!(Solution::three_sum(vec![0,0,0]), vec![vec![0,0,0]]);
    }
}
