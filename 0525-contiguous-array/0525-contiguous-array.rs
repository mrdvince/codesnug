impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 { 
        let mut map = std::collections::HashMap::with_capacity(nums.len());
        map.insert(0, -1);
        let mut running_sum = 0;
        let mut max_length = 0;
        
        for (i, &num) in nums.iter().enumerate() {
            if num == 0 {
                running_sum -= 1;
            } else {
                running_sum += 1;
            }
            
            match map.entry(running_sum) {
                std::collections::hash_map::Entry::Occupied(entry) => {
                    max_length = max_length.max(i as i32 - entry.get());
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(i as i32);
                }
            }
        }
        
        max_length
    }
}
