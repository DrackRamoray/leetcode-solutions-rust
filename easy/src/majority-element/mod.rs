pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut candidate = None;

        for &num in nums.iter() {
            if cnt == 0 {
                candidate = Some(num);
            }
            if let Some(v) = candidate {
                if v == num {
                    cnt += 1;
                } else {
                    cnt -= 1;
                }
            } else {
                cnt -= 1;
            }
        }

        candidate.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::majority_element(vec![3,2,3]), 3);
        assert_eq!(Solution::majority_element(vec![2,2,1,1,1,2,2]), 2);
    }
}
