pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let n = numbers.len();

        for i in 0..n {
            let mut lo = i + 1;
            let mut hi = n - 1;

            while lo <= hi && hi < n {
                let mid = lo + (hi - lo) / 2;

                if target - numbers[mid] == numbers[i] {
                    return vec![i as i32 + 1, mid as i32 + 1];
                }

                if target - numbers[mid] > numbers[i] {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![1,2]);
        assert_eq!(Solution::two_sum(vec![2,3,4], 6), vec![1,3]);
        assert_eq!(Solution::two_sum(vec![-1,0], -1), vec![1,2]);
    }
}
