use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_to_last_position: HashMap<char, usize> = HashMap::new();
        let mut res = 0;
        let mut left = 0;

        for (right, ch) in s.chars().enumerate() {
            if let Some(&last_pos) = char_to_last_position.get(&ch) {
                left = left.max(last_pos + 1);
            }
            char_to_last_position.insert(ch, right);
            res = res.max(right - left + 1);
        }
        res as i32
    }
}





// use std::collections::HashSet;

// impl Solution {
//     pub fn length_of_longest_substring(s: String) -> i32 {
//         let s = s.chars().collect::<Vec<char>>();
//         let mut char_set = HashSet::new();
//         let mut left_ptr = 0;
//         let mut res = 0;

//         for right_ptr in 0..s.len() {
//             while char_set.contains(&s[right_ptr]) {
//                 char_set.remove(&s[left_ptr]);
//                 left_ptr += 1;
//             }
//             char_set.insert(s[right_ptr]);
//             res = std::cmp::max(res, right_ptr - left_ptr + 1);
//         }
//         res as i32
//     }
// }