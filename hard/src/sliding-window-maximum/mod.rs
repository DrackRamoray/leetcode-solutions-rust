struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut queue = std::collections::VecDeque::new();

        for i in 0..k {
            while queue.len() > 0 && nums[i] > nums[queue[queue.len() - 1]] {
                queue.pop_back();
            }
            queue.push_back(i);
        }

        let mut ans = vec![nums[queue[0]]];

        for i in k..n {
            while queue.len() > 0 && nums[i] > nums[queue[queue.len() - 1]] {
                queue.pop_back();
            }
            queue.push_back(i);
            while queue[0] <= i - k {
                queue.pop_front();
            }
            ans.push(nums[queue[0]])
        }

        ans
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3), vec![3,3,5,5,6,7]);
    }
}
