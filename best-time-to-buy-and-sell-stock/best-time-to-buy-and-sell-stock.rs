impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut curr_profit, mut max_profit) = (0, 0);
        for index in 1..prices.len() {
            curr_profit = curr_profit + prices[index] - prices[index - 1];
            // if negative reset to zero
            curr_profit = std::cmp::max(0, curr_profit);
            max_profit = std::cmp::max(curr_profit, max_profit)
        }
        max_profit
    }
}
