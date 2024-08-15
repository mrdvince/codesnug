use std::collections::HashMap;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let mut s1_freq: HashMap<char, i32> = HashMap::new();
        let mut window_freq: HashMap<char, i32> = HashMap::new();

        // Frequency map for s1
        for ch in s1.chars() {
            *s1_freq.entry(ch).or_insert(0) += 1;
        }

        // Sliding window
        let s2 = s2.chars().collect::<Vec<char>>();
        for i in 0..s2.len() {
            *window_freq.entry(s2[i]).or_insert(0) += 1;
            // if window size > s1.len() remove left char
            if i >= s1.len() {
                let left_char = s2[i - s1.len()];
                *window_freq.get_mut(&left_char).unwrap() -= 1;
                if window_freq[&left_char] == 0 {
                    window_freq.remove(&left_char);
                }
            }
            if window_freq == s1_freq {
                return true
            }
        }
        false

    }
}