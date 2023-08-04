impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // DP
        let n = nums.len();
        let mut dp = vec![false;n];
        dp[0] = true;
        
        for i in 1..n {
            for j in 0..i {
                if dp[j] && j + nums[j] as usize >= i {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n-1]
    }
}