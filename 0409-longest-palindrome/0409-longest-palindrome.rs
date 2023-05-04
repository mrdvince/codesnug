impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut freqs = std::collections::HashMap::new();
        for c in s.chars() {
            *freqs.entry(c).or_insert(0) += 1;
        }
        let mut length = 0;
        let mut has_odd = false;
        for (_, count) in freqs {
            if count % 2 == 0 {
                length += count;
            } else {
                length += count - 1;
                has_odd = true;
            }
        }
        if has_odd {
            length += 1;
        }
        length
    }
}