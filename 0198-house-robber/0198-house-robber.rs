impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }

        let mut dp: Vec<i32> = vec![0; n];
        dp[0] = nums[0];
        dp[1] = std::cmp::max(nums[0], nums[1]);

        for i in 2..n {
            dp[i] = std::cmp::max(nums[i] + dp[i - 2], dp[i - 1]);
        }

        dp[n - 1]
    }
}
