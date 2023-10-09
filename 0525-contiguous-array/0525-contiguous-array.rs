impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        map.insert(0, -1);
        let mut running_sum = 0;
        let mut max_length = 0;

        for (i, &num) in nums.iter().enumerate() {
            if num == 0 {
                running_sum -= 1;
            } else {
                running_sum += 1;
            }
            if let Some(&first_occurence) = map.get(&running_sum) {
                max_length = max_length.max(i as i32 - first_occurence);
            } else {
                map.insert(running_sum, i as i32);
            }
        }
        max_length
    }
}