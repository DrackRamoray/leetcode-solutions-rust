struct Solution;

impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_by(|a, b| {
            let s1 = a.to_string() + &b.to_string();
            let s2 = b.to_string() + &a.to_string();

            s2.cmp(&s1)
        });

        if nums[0] == 0 {
            return "0".to_string();
        }

        nums.iter().map(|x| x.to_string()).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::largest_number(vec![10,2]), "210".to_string());
        assert_eq!(Solution::largest_number(vec![3,30,34,5,9]), "9534330".to_string());
    }
}
