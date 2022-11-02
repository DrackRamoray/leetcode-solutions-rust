pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 1;
        let mut j = 1;
        let n = nums.len();

        for i in 1..n {
            if nums[i] == nums[i-1] {
                count += 1;
            } else {
                count = 1;
            }

            if count <= 2 {
                nums[j] = nums[i];
                j += 1;
            }
        }

        return j as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1,1,1,2,2,3]), 5);
        assert_eq!(Solution::remove_duplicates(&mut vec![0,0,1,1,1,1,2,3,3]), 7);
    }
}
