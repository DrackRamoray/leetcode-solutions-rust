struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut ans = 1;

        for i in 0..n {
            let mut mp = std::collections::HashMap::new();
            let mut cnt = 0;

            for j in i+1..n {
                let x0 = points[i][0];
                let x1 = points[i][1];
                let y0 = points[j][0];
                let y1 = points[j][1];
                let a = x0 - y0;
                let b = x1 - y1;
                let k = Self::gcd(a, b);
                *mp.entry((a/k, b/k)).or_insert(0) += 1;
                if let Some(&num) = mp.get(&(a/k, b/k)) {
                    cnt = cnt.max(num);
                }
            }

            ans = ans.max(cnt + 1);
        }

        ans
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_points(vec![vec![1,1],vec![2,2],vec![3,3]]), 3);
        assert_eq!(Solution::max_points(vec![vec![1,1],vec![3,2],vec![5,3],vec![4,1],vec![2,3],vec![1,4]]), 4);
    }
}
