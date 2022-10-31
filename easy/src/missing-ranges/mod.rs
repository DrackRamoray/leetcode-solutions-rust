pub struct Solution;

impl Solution {
    pub fn find_missing_ranges(mut nums: Vec<i32>, mut lower: i32, upper: i32) -> Vec<String> {
        nums.push(upper + 1);
        lower -= 1;

        nums.iter().fold(Vec::new(), |mut acc, &cur| {
            let diff = cur - lower;

            if diff == 2 {
                acc.push((lower + 1).to_string());
            } else if diff > 2 {
                acc.push((lower + 1).to_string() + "->" + &(cur - 1).to_string());
            }

            lower = cur;

            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use assist::vec_stringify;
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_missing_ranges(vec![0,1,3,50,75], 0, 99), vec_stringify!(vec!["2", "4->49", "51->74", "76->99"]))
    }
}
