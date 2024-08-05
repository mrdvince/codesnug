impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let (mut left_ptr, mut right_ptr) = (0, 1);
        while right_ptr <= prices.len() - 1 {
            if prices[left_ptr] > prices[right_ptr] {
                left_ptr = right_ptr;
                right_ptr += 1;
            } else {
                max_profit = max_profit.max(prices[right_ptr] - prices[left_ptr]);
                right_ptr += 1;
            }
        }
        max_profit
    }
}
