impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        // If the total sum is odd, it cannot be partitioned into two subsets with equal sum.
        if total % 2 != 0 {
            return false; 
        }

        let half = total / 2;
        let mut dp = vec![vec![false; half as usize + 1]; nums.len() + 1];

        for i in 0..=nums.len() {
            // It's always possible to sum up to 0.
            dp[i][0] = true; 
        }

        for i in 1..=nums.len() {
            for j in 1..=half as usize {
                if nums[i - 1] as usize <= j {
                    dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i - 1] as usize];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
        
        dp[nums.len()][half as usize]
    }
}
