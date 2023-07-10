impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let max_value = amount + 1;
        let mut dp = vec![max_value; amount + 1];
        dp[0] = 0;
        
        for coin in coins {
            let coin = coin as usize;
            for i in coin..=amount {
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
        
        if dp[amount] > amount {
            return -1;
        }
        
        dp[amount] as i32
    }
}
