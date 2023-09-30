impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut max_length = 0;

        for &num in &nums {
            if !map.contains_key(&num) {
                let left = *map.get(&(num - 1)).unwrap_or(&0);
                let right = *map.get(&(num + 1)).unwrap_or(&0);

                let current_length = left + right + 1;
                map.insert(num, current_length);

                max_length = std::cmp::max(max_length, current_length);
                map.insert(num - left, current_length);
                map.insert(num + right, current_length);
            }
        }

        max_length
    }
}
