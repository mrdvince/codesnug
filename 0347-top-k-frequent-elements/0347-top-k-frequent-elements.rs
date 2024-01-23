use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }
        let mut counts: Vec<(i32, i32)> = counter.into_iter().collect();
        // Sort by frequency in descending order
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        println!("{:?}", counts);

        // Take top k elements
        counts
            .into_iter()
            .map(|(num, _)| num)
            .take(k as usize)
            .collect()
    }
}
