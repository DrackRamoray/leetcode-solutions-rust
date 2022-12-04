pub struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        for i in 0..nums.len() {
            let index = nums[i].abs() as usize - 1;

            if nums[index] > 0 {
                nums[index] = -nums[index];
            } else {
                ans.push(index as i32 + 1)
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::find_duplicates(vec![4,3,2,7,8,2,3,1]), vec![2,3]);
    assert_eq!(Solution::find_duplicates(vec![1,1,2]), vec![1]);
    assert_eq!(Solution::find_duplicates(vec![1]), vec![] as Vec<i32>);
}
