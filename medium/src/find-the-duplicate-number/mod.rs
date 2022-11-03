pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        slow = nums[slow as usize];
        fast = nums[nums[fast as usize] as usize];

        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
        }

        slow = 0;

        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }

        slow
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::find_duplicate(vec![1,3,4,2,2]), 2);
    assert_eq!(Solution::find_duplicate(vec![3,1,3,4,2]), 3);
}
