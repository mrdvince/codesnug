impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        // Map of char counts -> strings
        // {
        //  [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]: ["bat"],
        //  [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]: ["eat", "tea", "ate"],
        //  [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]: ["tan", "nat"]
        // }
        let mut counter: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            // 26 -> alpahabet
            // e.g [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
            let mut key = [0_u8; 26];
            for o in s.chars() {
                key[o as usize - 'a' as usize] += 1;
            }

            if let Some(values) = counter.get_mut(&key) {
                values.push(s);
            } else {
                counter.insert(key, vec![s]);
            }
        }

        counter.into_values().collect()
    }
}
