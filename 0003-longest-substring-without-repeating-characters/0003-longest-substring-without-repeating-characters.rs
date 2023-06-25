use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut char_set = HashSet::new();
        let mut left_ptr = 0;
        let mut res = 0;

        for right_ptr in 0..s.len() {
            while char_set.contains(&s[right_ptr]) {
                char_set.remove(&s[left_ptr]);
                left_ptr += 1;
            }
            char_set.insert(s[right_ptr]);
            res = std::cmp::max(res, right_ptr - left_ptr + 1);
        }
        res as i32
    }
}
