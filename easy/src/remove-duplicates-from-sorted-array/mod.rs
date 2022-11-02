struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();

        if n <= 1 {
            return n as i32;
        }

        let mut i = 1;

        for j in 1..n {
            if nums[j] != nums[j-1] {
                nums[i] = nums[j];
                i += 1;
            }
        }

        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1,1,2]), 2);
        assert_eq!(Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]), 5);
    }
}
