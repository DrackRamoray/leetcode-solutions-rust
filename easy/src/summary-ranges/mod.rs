pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let n = nums.len();
        let mut ans = vec![];
        let mut j = 0;

        for i in 0..n {
            if i == n - 1 || nums[i+1] != nums[i] + 1 {
                ans.push(if i == j { format!("{}", nums[i]) } else { format!("{}->{}", nums[j], nums[i]) });
                j = i + 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use assist::vec_stringify;
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::summary_ranges(vec![0,1,2,4,5,7]), vec_stringify!(vec!["0->2","4->5","7"]));
        assert_eq!(Solution::summary_ranges(vec![0,2,3,4,6,8,9]), vec_stringify!(vec!["0","2->4","6","8->9"]));
    }
}
