use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count_map: HashMap<char, i32> = HashMap::new();
        for (a, b) in s.chars().zip(t.chars()) {
            *count_map.entry(a).or_insert(0) += 1;
            *count_map.entry(b).or_insert(0) -= 1;
        }
        count_map.values().all(|&val| val == 0)
    }
}

// impl Solution {
//     pub fn is_anagram(s: String, t: String) -> bool {
//         if s.len() != t.len() {
//             return false;
//         }

//         let mut count_map = HashMap::new();
//         for ch in s.chars() {
//             *count_map.entry(ch).or_insert(0) += 1;
//         }
//         for ch in t.chars() {
//             let count = count_map.entry(ch).or_insert(0);
//             if *count == 0 {
//                 return false;
//             }
//             *count -= 1
//         }
//         true
//     }
// }
