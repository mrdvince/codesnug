use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut values = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            let curr_diff =  target - num;
            if let Some(j) = values.get(&curr_diff) {
                return vec![ *j as i32, idx as i32];
            }
            values.insert(num, idx);
        }
        vec![]
    }
}