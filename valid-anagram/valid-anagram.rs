use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut counts = HashMap::new();
        for ch in s.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        for ch in t.chars() {
            if let Some(count) = counts.get_mut(&ch) {
                *count -= 1;
                if *count < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }
        counts.values().all(|&count| count == 0)
    }
}
