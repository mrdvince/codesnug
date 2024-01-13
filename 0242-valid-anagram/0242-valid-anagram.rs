impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_vec:Vec<_> = s.chars().collect();
        s_vec.sort();
        let mut t_vec:Vec<_> = t.chars().collect();
        t_vec.sort();

        s_vec == t_vec
    }
}