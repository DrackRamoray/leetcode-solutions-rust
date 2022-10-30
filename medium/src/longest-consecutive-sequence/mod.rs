pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut num_set = std::collections::HashSet::new();

        for &num in nums.iter() {
            num_set.insert(num);
        }

        let mut ans = 0;

        for &num in num_set.iter() {
            if !num_set.contains(&(num - 1)) {
                let mut cur = num;
                let mut cnt = 1;

                while num_set.contains(&(cur + 1)) {
                    cur += 1;
                    cnt += 1;
                }

                ans = ans.max(cnt);
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
        assert_eq!(Solution::longest_consecutive(vec![100,4,200,1,3,2]), 4);
        assert_eq!(Solution::longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]), 9);
    }
}
