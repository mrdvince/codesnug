impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut clean_s = vec![];
        for c in s.chars() {
            if c.is_alphanumeric() {
                clean_s.push(c.to_lowercase().next().unwrap());
            }
        }
        let len = clean_s.len();
        for i in 0..len/2 {
            if clean_s[i] != clean_s[len - 1 - i] {
                return false
            }
        }
        true
    }
}