// impl Solution {
//     pub fn can_jump(nums: Vec<i32>) -> bool {
//         // DP
//         let n = nums.len();
//         let mut dp = vec![false;n];
//         dp[0] = true;
        
//         for i in 1..n {
//             for j in 0..i {
//                 if dp[j] && j + nums[j] as usize >= i {
//                     dp[i] = true;
//                     break;
//                 }
//             }
//         }
//         dp[n-1]
//     }
// }

// Greedy
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut reachable = 0;
        for i in 0..nums.len() {
            if i > reachable {
                return false;
            }
            reachable = reachable.max(i + nums[i] as usize);
            if reachable >= nums.len() - 1 {
                return true;
            }
        }
        true
    }
}