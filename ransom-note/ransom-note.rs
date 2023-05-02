impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_counts = [0; 26];
        for c in magazine.chars() {
            magazine_counts[c as usize - 'a' as usize] += 1;
        }
        for c in ransom_note.chars() {
            let count = &mut magazine_counts[c as usize - 'a' as usize];
            if *count == 0 {
                return false;
            }
            *count -= 1;
        }
        true
    }
}
