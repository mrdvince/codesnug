use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let s_counts = s.chars().fold(HashMap::new(), |mut counts, ch| {
            *counts.entry(ch).or_insert(0) += 1;
            counts
        });

        let t_counts = t.chars().fold(HashMap::new(), |mut counts, ch| {
            *counts.entry(ch).or_insert(0) += 1;
            counts
        });
        s_counts == t_counts
    }
}
