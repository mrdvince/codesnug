use std::collections::HashMap;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut count = HashMap::new();
        let (mut left, mut res) = (0, 0);

        for right in 0..s.len() {
            count.entry(s[right]).and_modify(|c| *c += 1).or_insert(1);
            let curr_dist = (right - left + 1) as i32;
            if curr_dist - *count.values().max().unwrap() > k {
                count.entry(s[left]).and_modify(|c| *c -= 1);
                left += 1
            } else {
                res = res.max(curr_dist);
            }
        }
        res
    }
}
