use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let chars: Vec<char> = s.chars().collect();
        let (mut ans, mut left) = (0, 0);
        let mut chars_set = HashSet::new();
        for (right, &c) in chars.iter().enumerate() {
            while chars_set.contains(&c) {
                chars_set.remove(&chars[left]);
                left += 1;
            }
            chars_set.insert(c);
            ans = ans.max(right - left + 1);
        }
        ans as i32
    }
}
