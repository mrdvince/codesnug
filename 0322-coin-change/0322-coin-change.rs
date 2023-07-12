impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // time complexity O(n*m) and space complexity O(n)
        let amount = amount as usize;
        let max = amount + 1;
        let mut dp = vec![max; max];
        dp[0] = 0;
        for i in 1..=amount {
            for &coin in &coins {
                let coin = coin as usize;
                if coin <= i {
                    dp[i] = dp[i].min(dp[i - coin] + 1);
                }
            }
        }
        if dp[amount] == max {
            return -1;
        }
        dp[amount] as i32
    }
}
