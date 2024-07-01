impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut new_s = vec![];
        for c in s.chars() {
            if c.is_alphanumeric() {
                new_s.push(c.to_lowercase().to_string())
            }
        }
        let mut left_ptr = 0;
        let mut right_ptr = new_s.len() -1;
        
        if new_s.is_empty() {
            return true;
        }

        while left_ptr < right_ptr {
            if new_s[left_ptr] != new_s[right_ptr]{
                return false;
            }
            left_ptr +=1;
            right_ptr -=1;
        }
        true
    }
}