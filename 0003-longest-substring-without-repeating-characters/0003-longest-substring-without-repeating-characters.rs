use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut chars_set = HashSet::new();
        for (right, c) in s.chars().enumerate() {
            while chars_set.contains(&c) {
                chars_set.remove(&s.chars().nth(left).unwrap());
                left += 1;
            }
            chars_set.insert(c);
            ans = ans.max(right - left + 1);
        }
        ans as i32
    }
}
