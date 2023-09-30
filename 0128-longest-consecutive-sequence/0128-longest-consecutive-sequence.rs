impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: std::collections::HashSet<i32> = nums.iter().cloned().collect();
        let mut max_streak = 0;

        for &num in &nums {
            if !num_set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_streak = 1;

                while num_set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }

                max_streak = std::cmp::max(max_streak, current_streak);
            }
        }

        max_streak
    }
}
