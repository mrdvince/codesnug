impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let non_dups: std::collections::HashSet<_> = nums.iter().collect();
        non_dups.len() != nums.len()
    }
}