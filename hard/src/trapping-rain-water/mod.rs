pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut ans = 0;
        let mut stack: Vec<usize> = vec![];

        for i in 0..n {
            while stack.len() > 0 && height[i] > height[stack[stack.len()-1]] {
                let top = stack.pop().unwrap();

                if stack.len() == 0 {
                    break;
                }

                let left = stack[stack.len() - 1];
                let w = i as i32 - left as i32 - 1;
                let h = height[left].min(height[i]) - height[top];
                ans += w * h;
            }

            stack.push(i);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
        assert_eq!(Solution::trap(vec![4,2,0,3,2,5]), 9);
    }
}
