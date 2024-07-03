impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(|c| c.to_lowercase())
            .collect();

        let length = s.len();
        for i in 0..(length / 2) {
            if s[i] != s[length - 1 - i] {
                return false;
            }
        }
        true
    }
}
