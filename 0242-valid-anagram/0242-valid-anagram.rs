use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count_map = HashMap::new();
        for ch in s.chars() {
            *count_map.entry(ch).or_insert(0) += 1;
        }
        for ch in t.chars(){
            let count = count_map.entry(ch).or_insert(0);
            if *count == 0 {
                return false;
            }
            *count -= 1
        }
        true
    }
}