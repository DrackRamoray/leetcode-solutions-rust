pub struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let n = nums.len();

        if n < 3 {
            return false;
        }

        let mut fst = nums[0];
        let mut snd = i32::MAX;

        for i in 1..n {
            if nums[i] > snd {
                return true;
            }

            if nums[i] > fst {
                snd = nums[i];
            } else {
                fst = nums[i];
            }
        }

        false
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::increasing_triplet(vec![1,2,3,4,5]), true);
    assert_eq!(Solution::increasing_triplet(vec![5,4,3,2,1]), false);
    assert_eq!(Solution::increasing_triplet(vec![2,1,5,0,4,6]), true);
}
