pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut arr = nums.clone();
        let len = nums.len();
        let mut ans = vec![];

        for i in 0..len {
            let cur = arr[i].abs() - 1;

            if arr[cur as usize] > 0 {
                arr[cur as usize] *= -1;
            }
        }

        for i in 0..len {
            if arr[i] > 0 {
                ans.push(i as i32+1);
            }
        }

        return ans;
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]), vec![5,6]);
    assert_eq!(Solution::find_disappeared_numbers(vec![1,1]), vec![2]);
}
