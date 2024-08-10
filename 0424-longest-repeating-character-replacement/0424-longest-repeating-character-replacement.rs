use std::collections::HashMap;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut count = HashMap::new();
        let (mut left, mut res, mut max_freq) = (0, 0, 0);

        for right in 0..s.len() {
            *count.entry(s[right]).or_insert(0) += 1;

            max_freq = max_freq.max(*count.get(&s[right]).unwrap());

            if (right - left + 1) as i32 - max_freq > k {
                *count.get_mut(&s[left]).unwrap() -= 1;
                left += 1
            }
            res = res.max(right - left + 1);
        }
        res as i32
    }
}
