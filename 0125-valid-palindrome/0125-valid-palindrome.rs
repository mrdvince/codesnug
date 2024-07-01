impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut new_s: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(|c| c.to_lowercase())
            .collect();

        let mut left_ptr = 0;
        let mut right_ptr = new_s.len() - 1;

        if new_s.is_empty() {
            return true;
        }

        while left_ptr < right_ptr {
            if new_s[left_ptr] != new_s[right_ptr] {
                return false;
            }
            left_ptr += 1;
            right_ptr -= 1;
        }
        true
    }
}
