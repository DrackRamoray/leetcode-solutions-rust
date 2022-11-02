struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut min_price = i32::MAX;

        for price in prices.into_iter() {
            if price < min_price {
                min_price = price;
            }

            ans = ans.max(price - min_price);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 5);
        assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0);
    }
}
