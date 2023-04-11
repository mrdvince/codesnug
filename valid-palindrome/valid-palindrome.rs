impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut clean_s = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<_>>();

        let len = clean_s.len();
        for i in 0..len / 2 {
            if clean_s[i] != clean_s[len - 1 - i] {
                return false;
            }
        }
        true
    }
}
