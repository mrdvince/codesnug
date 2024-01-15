use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        for (idx, num) in nums.into_iter().enumerate() {
            let diff = target - num;
            if let Some(&j) = seen.get(&diff) {
                return vec![idx as i32, j as i32];
            } else {
                seen.insert(num, idx);
            }
        }
        vec![]
    }
}
