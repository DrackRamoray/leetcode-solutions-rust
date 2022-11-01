pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut major1 = nums[0];
        let mut count1 = 0_i32;

        let mut major2 = nums[0];
        let mut count2 = 0_i32;

        for &num in nums.iter() {
            if major1 == num {
                count1 += 1;
                continue;
            }

            if major2 == num {
                count2 += 1;
                continue;
            }

            if count1 == 0 {
                major1 = num;
                count1 += 1;
                continue;
            }

            if count2 == 0 {
                major2 = num;
                count2 += 1;
                continue;
            }

            count1 -= 1;
            count2 -= 1;
        }

        count1 = 0;
        count2 = 0;

        for &num in nums.iter() {
            if num == major1 {
                count1 += 1;
            }
            if num == major2 {
                count2 += 1;
            }
        }

        let n = (nums.len() / 3) as i32;
        let mut ans = Vec::new();

        if count1 > n {
            ans.push(major1);
        }

        if count2 > n && major1 != major2 {
            ans.push(major2);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::majority_element(vec![3,2,3]), vec![3]);
        assert_eq!(Solution::majority_element(vec![1]), vec![1]);
        assert_eq!(Solution::majority_element(vec![1,2]), vec![1,2]);
    }
}
