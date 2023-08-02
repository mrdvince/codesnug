impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut memo = vec![vec![-1; nums.len()]; nums.len() + 1];
        Solution::lis(&nums, -1, 0, &mut memo)
    }
    
    fn lis(nums: &Vec<i32>, prev_index: i32, curr_index: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if curr_index == nums.len() {
            return 0;
        }

        if prev_index >= 0 && memo[(prev_index + 1) as usize][curr_index] >= 0 {
            return memo[(prev_index + 1) as usize][curr_index];
        }

        let taken = if prev_index < 0 || nums[curr_index] > nums[prev_index as usize] {
            1 + Solution::lis(nums, curr_index as i32, curr_index + 1, memo)
        } else {
            0
        };
        
        let not_taken = Solution::lis(nums, prev_index, curr_index + 1, memo);

        memo[(prev_index + 1) as usize][curr_index] = std::cmp::max(taken, not_taken);
        memo[(prev_index + 1) as usize][curr_index]
    }
}
