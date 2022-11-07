pub struct Solution;

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut m = std::collections::HashMap::new();
        m.insert(0, -1);
        let mut ans = 0;
        let mut pre_sum = 0;

        for i in 0..len {
            pre_sum += nums[i];

            if !m.contains_key(&pre_sum) {
                m.insert(pre_sum, i as i32);
            }

            if let Some(&v) = m.get(&(pre_sum - k)) {
                ans = ans.max(i as i32 - v);
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::max_sub_array_len(vec![1,-1,5,-2,3], 3), 4);
    assert_eq!(Solution::max_sub_array_len(vec![-2,-1,2,1], 1), 2);
}
