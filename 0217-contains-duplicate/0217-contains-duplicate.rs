// O(n) -> time
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut non_dups = std::collections::HashSet::new();

        for num in nums.iter() {
            if !non_dups.insert(num) {
                return true;
            }
        }
        return false;
    }
}

// O(n) -> time 
// O(min(n,k)) where k is number of unique -> space
// impl Solution {
//     pub fn contains_duplicate(nums: Vec<i32>) -> bool {
//         let non_dups: std::collections::HashSet<_> = nums.iter().collect();
//         non_dups.len() != nums.len()
//     }
// }