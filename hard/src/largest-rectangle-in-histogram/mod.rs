pub struct  Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut left = vec![-1;n];
        let mut right = vec![n as i32;n];
        let mut stack = vec![];

        for i in 0..n {
            while stack.len() > 0 && heights[stack[stack.len()-1]] >= heights[i] {
                stack.pop();
            }
            if stack.len() != 0 {
                left[i] = stack[stack.len()-1] as i32;
            }
            stack.push(i);
        }

        stack = vec![];

        for i in (0..n).rev() {
            while stack.len() > 0 && heights[stack[stack.len()-1]] >= heights[i] {
                stack.pop();
            }
            if stack.len() != 0 {
                right[i] = stack[stack.len()-1] as i32;
            }
            stack.push(i);
        }

        let mut ans = 0;

        for i in 0..n {
            ans = ans.max((right[i] - left[i] - 1) * heights[i]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::largest_rectangle_area(vec![2,1,5,6,2,3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2,4]), 4);
    }
}
