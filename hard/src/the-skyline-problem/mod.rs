pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points = vec![];

        for building in buildings.iter() {
            points.push((building[0], -building[2]));
            points.push((building[1], building[2]));
        }

        points.sort();

        let mut queue = BinaryHeap::new();
        let mut prev = 0;
        let mut ans = vec![];

        queue.push(prev);

        for point in points.iter() {
            if point.1 < 0 {
                queue.push(-point.1);
            } else {
                let mut tmp = queue.into_vec();
                for index in 0..tmp.len() {
                    if tmp[index] == point.1 {
                        tmp.remove(index);
                        break;
                    }
                }
                queue = tmp.into_iter().collect();
            }

            if let Some(&cur) = queue.peek() {
                if cur != prev {
                    ans.push(vec![point.0, cur]);
                    prev = cur;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::get_skyline(vec![vec![2,9,10],vec![3,7,15],vec![5,12,12],vec![15,20,10],vec![19,24,8]]), vec![vec![2,10],vec![3,15],vec![7,12],vec![12,0],vec![15,10],vec![20,8],vec![24,0]]);
        assert_eq!(Solution::get_skyline(vec![vec![0,2,3],vec![2,5,3]]), vec![vec![0,3],vec![5,0]]);
    }
}
