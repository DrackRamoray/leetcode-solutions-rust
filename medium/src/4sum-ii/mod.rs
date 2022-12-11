pub struct Solution;

impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut mp = std::collections::HashMap::new();

        for &num3 in nums3.iter() {
            for &num4 in nums4.iter() {
                *mp.entry(num3 + num4).or_insert(0) += 1;
            }
        }

        let mut ans = 0;

        for &num1 in nums1.iter() {
            for &num2 in nums2.iter() {
                if let Some(num) = mp.get(&(-num1-num2)) {
                    ans += num;
                }
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::four_sum_count(vec![1,2], vec![-2,-1], vec![-1,2], vec![0,2]), 2);
    assert_eq!(Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0]), 1);
}
