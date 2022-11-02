struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();

        let mut i = n - 2;

        while i < n - 1 && nums[i] >= nums[i+1] {
            i = i.wrapping_sub(1);
        }

        if i < n-1 {
            let mut j = n - 1;

            while j < n && nums[i] >= nums[j] {
                j -= 1;
            }

            nums.swap(i, j);
            nums[i+1..].reverse();
        } else {
            nums.reverse();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut nums = vec![1,2,3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1,3,2]);

        let mut nums = vec![3,2,1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1,2,3]);

        nums = vec![1,1,5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1,5,1]);
    }
}
