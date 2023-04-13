use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        // Create a collections counter equivalent
        let mut counts = HashMap::new();

        for ch in s.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            if let Some(count) = counts.get_mut(&ch) {
                *count -= 1;
                if *count == 0 {
                    counts.remove(&ch);
                }
            } else {
                return false;
            }
        }
        counts.is_empty()
    }
}
