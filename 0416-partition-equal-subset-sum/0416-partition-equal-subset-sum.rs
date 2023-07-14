impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let target = sum / 2;
        let mut memo = vec![vec![None; target as usize + 1]; nums.len()];

        Solution::can_partition_recursive(&nums, target, 0, &mut memo)
    }

    fn can_partition_recursive(nums: &Vec<i32>, sum: i32, index: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
        if sum == 0 {
            return true;
        }

        if index >= nums.len() || sum < 0 {
            return false;
        }

        if let Some(memo_val) = memo[index][sum as usize] {
            return memo_val;
        }

        if nums[index] <= sum {
            if Solution::can_partition_recursive(nums, sum - nums[index], index + 1, memo) {
                memo[index][sum as usize] = Some(true);
                return true;
            }
        }

        memo[index][sum as usize] = Some(Solution::can_partition_recursive(nums, sum, index + 1, memo));
        memo[index][sum as usize].unwrap()
    }
}


// impl Solution {
//     pub fn can_partition(nums: Vec<i32>) -> bool {
//         let total: i32 = nums.iter().sum();
//         // If the total sum is odd, it cannot be partitioned into two subsets with equal sum.
//         if total % 2 != 0 {
//             return false; 
//         }

//         let half = total / 2;
//         let mut dp = vec![vec![false; half as usize + 1]; nums.len() + 1];

//         for i in 0..=nums.len() {
//             // It's always possible to sum up to 0.
//             dp[i][0] = true; 
//         }

//         for i in 1..=nums.len() {
//             for j in 1..=half as usize {
//                 if nums[i - 1] as usize <= j {
//                     dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i - 1] as usize];
//                 } else {
//                     dp[i][j] = dp[i - 1][j];
//                 }
//             }
//         }
        
//         dp[nums.len()][half as usize]
//     }
// }
