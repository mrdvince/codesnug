impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        if nums.len() == 0 {
            return 0;
        }
        let num_set: HashSet<i32> = nums.into_iter().collect();
        println!("{num_set:?}");
        let mut longest = 0;
        for &num in num_set.iter() {
            if !num_set.contains(&(num - 1)) {
                let mut curr_num = num;
                let mut curr_streak = 1;

                while num_set.contains(&(curr_num + 1)) {
                    curr_num += 1;
                    curr_streak += 1;
                }
                longest = longest.max(curr_streak);
            }
        }
        longest
    }
}
