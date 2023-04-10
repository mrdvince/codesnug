impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut curr_profit, mut max_profit) = (0, 0);
        for index in 1..prices.len() {
            // if negative reset to zero
            curr_profit = curr_profit.max(0) + prices[index] - prices[index - 1];
            max_profit = max_profit.max(curr_profit);
        }
        max_profit
    }
}
