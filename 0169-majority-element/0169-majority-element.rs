impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut elements_counter = std::collections::HashMap::new();
        for num in &nums {
            *elements_counter.entry(num).or_insert(0) += 1;
        }
        let mut ans = 0;
        let mut seen = 0;
        for (&k, v) in elements_counter {
            if v > nums.len() / 2 && v >= seen {
                ans = k;
                seen = v;
            }
        }
        ans
    }
}